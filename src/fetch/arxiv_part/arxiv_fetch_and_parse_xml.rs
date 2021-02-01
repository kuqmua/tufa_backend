// use reqwest;
use serde_xml_rs::from_str;
use std::collections::HashMap;
use std::time::Instant;

use super::arxiv_structures::ArxivPost;
use super::arxiv_structures::ArxivPostStruct;
use super::arxiv_structures::Creator;
use super::arxiv_structures::XmlArxivParserStruct;
use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::overriding::prints::print_error_red;

pub enum HandledFetchStatusInfo {
    Initialized,
    ResToTextError(String),
    ResStatusError(reqwest::StatusCode),
    Success,
}
pub enum UnhandledFetchStatusInfo {
    Failure(String),
    Initialized,
    Success,
}
pub enum AreThereItems {
    //модет быть parse error
    Yep,
    Initialized,
    NopeButThereIsTag(String),
    ConversionFromStrError(String, String),
    NopeNoTag(String),
}

pub fn do_something() -> HashMap<
    String,
    (
        ArxivPostStruct,
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
    ),
> {
    let time = Instant::now();
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links();
    let mut hashmap_to_return: HashMap<
        String,
        (
            ArxivPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
        ),
    > = HashMap::new();
    for (key, value) in arxiv_links_in_hash_map {
        let useless_arxiv_page_struct = ArxivPostStruct::new();
        let tuple = (
            useless_arxiv_page_struct,
            value.to_string(),
            UnhandledFetchStatusInfo::Initialized,
            HandledFetchStatusInfo::Initialized,
            AreThereItems::Initialized,
        );
        hashmap_to_return.insert(key.to_string(), tuple);
    }
    if ENABLE_PRINTS_ARXIV {
        println!(
            "hashmap init in {}.{}ms",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
        );
    };
    let crossbeam_result = crossbeam::scope(|scope| {
        for (key, value) in &mut hashmap_to_return {
            scope.spawn(move |_| {
                let fetch_result = fetch_link(&value.1, key, time);
                match fetch_result {
                    Ok(fetch_tuple_result) => {
                        value.2 = UnhandledFetchStatusInfo::Success;
                        match fetch_tuple_result.1 {
                            HandledFetchStatusInfo::Initialized => {
                                value.3 = HandledFetchStatusInfo::Initialized;
                            }
                            HandledFetchStatusInfo::ResToTextError(res_to_text_string_error) => {
                                value.3 = HandledFetchStatusInfo::ResToTextError(
                                    res_to_text_string_error,
                                );
                            }
                            HandledFetchStatusInfo::ResStatusError(res_error_code) => {
                                value.3 = HandledFetchStatusInfo::ResStatusError(res_error_code);
                            }
                            HandledFetchStatusInfo::Success => {
                                let since_fetch = Instant::now();
                                value.3 = HandledFetchStatusInfo::Success;
                                let (arxiv_post_struct_handle, are_there_items_handle) =
                                    parse_string_into_struct(fetch_tuple_result.0, key, &value.1);
                                value.0 = arxiv_post_struct_handle;
                                value.4 = are_there_items_handle;
                                if ENABLE_PRINTS_ARXIV {
                                    println!(
                                        "parse in {}.{}ms abs, rel {}.{}ms for {}",
                                        time.elapsed().as_secs(),
                                        time.elapsed().as_millis() / 10,
                                        since_fetch.elapsed().as_secs(),
                                        since_fetch.elapsed().as_millis() / 10,
                                        key
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        value.2 = UnhandledFetchStatusInfo::Failure(e.to_string()); // add e
                        if ENABLE_ERROR_PRINTS_ARXIV {
                            print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                        }
                    }
                }
            });
        }
    });
    match crossbeam_result {
        Ok(_) => {
            if ENABLE_PRINTS_ARXIV {
                println!("crossbeam_result is ok",)
            }
        }
        Err(e) => {
            if ENABLE_ERROR_PRINTS_ARXIV {
                eprintln!(
                    "crossbeam_result is not ok, file: {}, line {}\n {:#?}",
                    file!().to_string(),
                    line!().to_string(),
                    e
                )
            }
        }
    }
    // println!("arxiv_sections_links {:#?}", hashmap_to_return);
    hashmap_to_return
}

fn fetch_link(
    link: &str,
    key: &str,
    time: Instant,
) -> Result<(String, HandledFetchStatusInfo), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(link)?;
    if ENABLE_PRINTS_ARXIV {
        println!(
            "fetch in {}.{}ms... status {} for {}",
            time.elapsed().as_secs(),
            time.elapsed().as_millis() / 10,
            res.status(),
            key
        );
    }
    let mut result_tuple: (String, HandledFetchStatusInfo) =
        ("".to_string(), HandledFetchStatusInfo::Initialized);
    if res.status() == reqwest::StatusCode::OK {
        let res_to_text_result = res.text();
        match res_to_text_result {
            Ok(norm) => result_tuple = (norm, HandledFetchStatusInfo::Success),
            Err(e) => {
                result_tuple.1 = HandledFetchStatusInfo::ResToTextError(e.to_string());
                if ENABLE_ERROR_PRINTS_ARXIV {
                    print_error_red(file!().to_string(), line!().to_string(), e.to_string());
                }
            }
        }
    } else {
        result_tuple.1 = HandledFetchStatusInfo::ResStatusError(res.status());
        if ENABLE_ERROR_PRINTS_ARXIV {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                res.status().to_string(),
            );
        }
    }
    Ok(result_tuple)
}

fn parse_string_into_struct(
    fetch_tuple_result: String,
    key: &str,
    value: &str,
) -> (ArxivPostStruct, AreThereItems) {
    let mut arxiv_post_struct_handle: ArxivPostStruct = ArxivPostStruct::new();
    let are_there_items_handle: AreThereItems; // = AreThereItems::Initialized
    match fetch_tuple_result.find("</item>") {
        Some(_) => {
            let arxiv_struct_from_str_result: Result<XmlArxivParserStruct, serde_xml_rs::Error> =
                from_str(&fetch_tuple_result);
            match arxiv_struct_from_str_result {
                Ok(arxiv_struct) => {
                    let mut count = 0;
                    let mut arxiv_page_struct: ArxivPostStruct = ArxivPostStruct::new();
                    loop {
                        if count < arxiv_struct.items.len() {
                            let mut arxiv_post: ArxivPost = ArxivPost::new();
                            arxiv_post.title = arxiv_struct.items[count].title.clone();
                            arxiv_post.link = arxiv_struct.items[count].link.clone();
                            arxiv_post.description = arxiv_struct.items[count].description.clone();
                            let mut string_part_for_loop =
                                arxiv_struct.items[count].creator.clone();
                            while let Some(link_index_from_start) =
                                string_part_for_loop.find("<a href=\"")
                            {
                                if let Some(link_index_from_end) = string_part_for_loop.find("\">")
                                {
                                    if let Some(name_index_from_end) =
                                        string_part_for_loop.find("</a>")
                                    {
                                        let mut creator = Creator::new();
                                        creator.link = string_part_for_loop[link_index_from_start
                                            + "<a href=\"".len()
                                            ..link_index_from_end]
                                            .to_string();
                                        let name_index_from_start =
                                            link_index_from_end + "\">".len();
                                        creator.name = string_part_for_loop
                                            [name_index_from_start..name_index_from_end]
                                            .to_string();
                                        string_part_for_loop = string_part_for_loop
                                            [name_index_from_end + "\">".len()..]
                                            .to_string();
                                        arxiv_post.creators.push(creator);
                                    }
                                }
                            }

                            arxiv_page_struct.items.push(arxiv_post);
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if !arxiv_page_struct.items.is_empty() {
                        are_there_items_handle = AreThereItems::Yep;
                    } else {
                        are_there_items_handle =
                            AreThereItems::NopeButThereIsTag(fetch_tuple_result);
                    }
                    arxiv_post_struct_handle = arxiv_page_struct;
                }
                Err(e) => {
                    if ENABLE_ERROR_PRINTS_ARXIV {
                        println!(
                            "arxiv conversion from str for {}, error {}",
                            key,
                            e.to_string()
                        );
                    };
                    are_there_items_handle =
                        AreThereItems::ConversionFromStrError(fetch_tuple_result, e.to_string());
                }
            }
        }
        _ => {
            if ENABLE_PRINTS_ARXIV {
                println!("arxiv no items for key {} {}", key, value);
            };
            are_there_items_handle = AreThereItems::NopeNoTag(fetch_tuple_result);
        }
    }
    (arxiv_post_struct_handle, are_there_items_handle)
}
