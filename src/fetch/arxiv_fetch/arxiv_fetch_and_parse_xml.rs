use reqwest;
use serde_xml_rs::from_str;
use std::collections::HashMap;
// use std::str;
use std::time::Instant;

use super::arxiv_structures::ArxivPost;
use super::arxiv_structures::ArxivPostStruct;
use super::arxiv_structures::Creator;
use super::arxiv_structures::XmlArxivParserStruct;
use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::override_prints::override_prints::print_error_red;

pub fn do_something() -> HashMap<String, (String, ArxivPostStruct)> {
    let time = Instant::now();
    let res_text_error: &str = "res.text() error";
    let res_status_error: &str = "fetch_link status: NOT OK ";
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links();
    let mut hashmap_to_return: HashMap<String, (String, ArxivPostStruct)> = HashMap::new();
    for (key, value) in arxiv_links_in_hash_map {
        let useless_arxiv_page_struct = ArxivPostStruct::new();
        let tuple = (value.to_string(), useless_arxiv_page_struct);
        hashmap_to_return.insert(key.to_string(), tuple);
    }
    let crossbeam_result = crossbeam::scope(|scope| {
        for (key, value) in &mut hashmap_to_return {
            scope.spawn(move |_| {
                let fetch_result =
                    fetch_link(&value.0, key, time, res_text_error, res_status_error);
                match fetch_result {
                    Ok(fetch_string_result) => {
                        //add better error handling
                        if fetch_string_result == res_text_error {
                            value.1.title = res_text_error.to_string(); //костыль, переделать позже структуру
                        } else if fetch_string_result == res_status_error {
                            value.1.title = res_text_error.to_string(); //костыль, переделать позже структуру
                        } else {
                            match fetch_string_result.find("</item>") {
                                Some(_) => {
                                    let arxiv_struct: XmlArxivParserStruct =
                                        from_str(&fetch_string_result).unwrap();
                                    let mut count = 0;
                                    let mut arxiv_page_struct: ArxivPostStruct =
                                        ArxivPostStruct::new();
                                    loop {
                                        if count < arxiv_struct.items.len() {
                                            let mut arxiv_post: ArxivPost = ArxivPost::new();
                                            arxiv_post.title =
                                                arxiv_struct.items[count].title.clone();
                                            arxiv_post.link =
                                                arxiv_struct.items[count].link.clone();
                                            arxiv_post.description =
                                                arxiv_struct.items[count].description.clone();
                                            let mut string_part_for_loop =
                                                arxiv_struct.items[count].creator.clone();
                                            while let Some(link_index_from_start) =
                                                string_part_for_loop.find("<a href=\"")
                                            {
                                                if let Some(link_index_from_end) =
                                                    string_part_for_loop.find("\">")
                                                {
                                                    if let Some(name_index_from_end) =
                                                        string_part_for_loop.find("</a>")
                                                    {
                                                        let mut creator = Creator::new();
                                                        creator.link = string_part_for_loop
                                                            [link_index_from_start
                                                                + "<a href=\"".len()
                                                                ..link_index_from_end]
                                                            .to_string();
                                                        let name_index_from_start =
                                                            link_index_from_end + "\">".len();
                                                        creator.name = string_part_for_loop
                                                            [name_index_from_start
                                                                ..name_index_from_end]
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
                                    value.1 = arxiv_page_struct;
                                }
                                _ => {
                                    if ENABLE_PRINTS_ARXIV {
                                        println!("arxiv no items for key {} {}", key, value.0);
                                    };
                                    value.1.title = "no items for key".to_string();
                                    //костыль, переделать позже структуру
                                }
                            }
                            println!(
                                "parse in {}.{}ms _____________ for {}",
                                time.elapsed().as_secs(),
                                time.elapsed().as_millis(),
                                key
                            );
                        }
                    }
                    Err(e) => {
                        print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                    }
                }
            });
        }
    });
    // println!("arxiv_sections_links {:#?}", hashmap_to_return);
    hashmap_to_return
}

fn fetch_link(
    link: &str,
    key: &str,
    time: Instant,
    res_text_error: &str,
    res_status_error: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(link)?;
    println!(
        "fetch in {}.{}ms status {} for {}",
        time.elapsed().as_secs(),
        time.elapsed().as_millis(),
        res.status(),
        key
    );
    let b: String;
    if res.status() == reqwest::StatusCode::OK {
        let s = res.text();
        match s {
            Ok(norm) => b = norm,
            Err(e) => {
                b = res_text_error.to_string();
                print_error_red(file!().to_string(), line!().to_string(), e.to_string());
            }
        }
    } else {
        b = res_status_error.to_string();
        print_error_red(
            file!().to_string(),
            line!().to_string(),
            res.status().to_string(),
        );
    }
    Ok(b)
}
