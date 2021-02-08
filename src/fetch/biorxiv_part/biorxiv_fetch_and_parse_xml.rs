// use futures::future;
// use reqwest::Client;
// use serde_xml_rs::from_str;
// use std::collections::HashMap;
// use std::str;
// use std::time::Instant;
// use tokio;

// use super::biorxiv_structures::BiorxivPageStruct;
// use super::biorxiv_structures::XmlBiorxivParserStruct;
// use crate::config::ENABLE_ERROR_PRINTS_BIORXIV;
// use crate::config::ENABLE_PRINTS_BIORXIV;
// use crate::override_prints::override_prints::print_error_red;

// #[tokio::main]
// pub async fn biorxiv_fetch_and_parse_xml(
//     vec_of_links: Vec<&str>,
//     vec_of_keys: Vec<&str>,
// ) -> HashMap<String, BiorxivPageStruct> {
//     let time = Instant::now();
//     let mut biorxiv_structs_vec: HashMap<String, BiorxivPageStruct> =
//         HashMap::with_capacity(vec_of_links.len());
//     let client = Client::new();
//     let bodies = future::join_all(vec_of_links.into_iter().map(|url| {
//         let client = &client;
//         async move {
//             let resp = client.get(url).send().await?;
//             resp.bytes().await
//         }
//     }))
//     .await;
//     if ENABLE_PRINTS_BIORXIV {
//         println!(
//             "fetch in {} seconds, biorxiv bodies: {} ",
//             time.elapsed().as_secs(),
//             bodies.len()
//         );
//     }
//     let mut key_count = 0;
//     for b in bodies {
//         match b {
//             Ok(b) => {
//                 let slice: &[u8] = &b;
//                 let converted_str = str::from_utf8(&slice).unwrap();
//                 let mut dots_unfiltered_str = converted_str.to_string();
//                 // расписать случай если не найдет посты
//                 let mut count_for_items = 0;
//                 loop {
//                     match dots_unfiltered_str.find("<dc:title>") {
//                         Some(_) => match dots_unfiltered_str.find("</dc:title>") {
//                             Some(_) => {
//                                 dots_unfiltered_str =
//                                     dots_unfiltered_str.replace("<dc:title>", "<dcstitle>");
//                                 dots_unfiltered_str =
//                                     dots_unfiltered_str.replace("</dc:title>", "</dcstitle>");
//                                 count_for_items += 1;
//                             }
//                             _ => {
//                                 break;
//                             }
//                         },
//                         _ => {
//                             break;
//                         }
//                     }
//                 }
//                 if count_for_items > 0 {
//                     let biorvix_struct: XmlBiorxivParserStruct =
//                         from_str(&dots_unfiltered_str).unwrap();
//                     let mut count = 0;
//                     let mut biorxiv_page_struct: BiorxivPageStruct = BiorxivPageStruct::new();
//                     let mut xml_parser_one_string_creators =
//                         biorvix_struct.items[count].creator.clone();
//                     loop {
//                         if count < biorvix_struct.items.len() {
//                             biorxiv_page_struct.items[count].title =
//                                 biorvix_struct.items[count].title.clone();
//                             biorxiv_page_struct.items[count].link =
//                                 biorvix_struct.items[count].link.clone();
//                             biorxiv_page_struct.items[count].description =
//                                 biorvix_struct.items[count].description.clone();
//                             match xml_parser_one_string_creators.find("., ") {
//                                 Some(end_of_creator) => {
//                                     biorxiv_page_struct.items[count].creators.push(
//                                         xml_parser_one_string_creators[..end_of_creator]
//                                             .to_string(),
//                                     );
//                                     xml_parser_one_string_creators = xml_parser_one_string_creators
//                                         [end_of_creator + "., ".len()..]
//                                         .to_string();
//                                 }
//                                 None => {
//                                     biorxiv_page_struct.items[count]
//                                         .creators
//                                         .push(xml_parser_one_string_creators.clone());
//                                     break;
//                                 }
//                             }
//                             biorxiv_page_struct.items[count].date =
//                                 biorvix_struct.items[count].date.clone();
//                             biorxiv_page_struct.items[count].publisher =
//                                 biorvix_struct.items[count].publisher.clone();
//                             count += 1;
//                         } else {
//                             break;
//                         }
//                     }
//                     biorxiv_structs_vec
//                         .insert(vec_of_keys[key_count].to_string(), biorxiv_page_struct);
//                 } else {
//                     if ENABLE_PRINTS_BIORXIV {
//                         println!("(biorxiv) no items for key {}", vec_of_keys[key_count]);
//                     }
//                     let useless_biorxiv_page_struct = BiorxivPageStruct::new();
//                     biorxiv_structs_vec.insert(
//                         vec_of_keys[key_count].to_string(),
//                         useless_biorxiv_page_struct,
//                     );
//                 }
//             }
//             Err(e) => {
//                 if ENABLE_ERROR_PRINTS_BIORXIV {
//                     print_error_red(
//                         file!().to_string(),
//                         line!().to_string(),
//                         e.to_string() + &"\nkey count: ".to_string() + &key_count.to_string(),
//                     )
//                 }
//             }
//         }
//         key_count += 1;
//     }
//     if ENABLE_PRINTS_BIORXIV {
//         println!(
//             "parsing in {} seconds, biorxiv bodies: {} ",
//             time.elapsed().as_secs(),
//             biorxiv_structs_vec.len()
//         );
//     }
//     biorxiv_structs_vec.clone()
// }

use std::collections::HashMap;
use std::time::Instant;

use super::biorxiv_check_handled_fetch_status_info::check_handled_fetch_status_info;
use super::biorxiv_fetch_link::biorxiv_fetch_link;
use super::biorxiv_metainfo_structures::AreThereItems;
use super::biorxiv_metainfo_structures::HandledFetchStatusInfo;
use super::biorxiv_metainfo_structures::UnhandledFetchStatusInfo;
use super::biorxiv_structures::BiorxivPageStruct; //page instead of post wtf????

use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::overriding::prints::print_error_red;

pub fn biorxiv_fetch_and_parse_xml() -> HashMap<
    String,
    (
        BiorxivPageStruct,
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
    ),
> {
    println!("ffffff");
    let time = Instant::now();
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links();
    let mut hashmap_to_return: HashMap<
        String,
        (
            BiorxivPageStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
        ),
    > = HashMap::new();
    for (key, value) in arxiv_links_in_hash_map {
        let tuple = (
            BiorxivPageStruct::new(),
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
                let fetch_result = biorxiv_fetch_link(&value.1, key, time);
                match fetch_result {
                    Ok(fetch_tuple_result) => {
                        value.2 = UnhandledFetchStatusInfo::Success;
                        let (
                            value3,
                            arxiv_post_struct_wrapper_handle,
                            are_there_items_wrapper_handle,
                        ) = check_handled_fetch_status_info(
                            fetch_tuple_result.1,
                            fetch_tuple_result.0,
                            time,
                            key,
                            &value.1,
                        );
                        value.3 = value3;
                        value.0 = arxiv_post_struct_wrapper_handle;
                        value.4 = are_there_items_wrapper_handle;
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
