use futures::future;
use reqwest::Client;
use serde_xml_rs::from_str;
use std::collections::HashMap;
use std::str;
use std::time::Instant;
use tokio;

use super::biorxiv_structures::BiorxivPageStruct;
use super::biorxiv_structures::XmlBiorxivParserStruct;
use crate::config::ENABLE_ERROR_PRINTS_BIORXIV;
use crate::config::ENABLE_PRINTS_BIORXIV;
use crate::override_prints::override_prints::print_error_red;

#[tokio::main]
pub async fn biorxiv_fetch_and_parse_xml(
    vec_of_links: Vec<&str>,
    vec_of_keys: Vec<&str>,
) -> HashMap<String, BiorxivPageStruct> {
    let time = Instant::now();
    let mut biorxiv_structs_vec: HashMap<String, BiorxivPageStruct> =
        HashMap::with_capacity(vec_of_links.len());
    let client = Client::new();
    let bodies = future::join_all(vec_of_links.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    }))
    .await;
    if ENABLE_PRINTS_BIORXIV {
        println!(
            "fetch in {} seconds, biorxiv bodies: {} ",
            time.elapsed().as_secs(),
            bodies.len()
        );
    }
    let mut key_count = 0;
    for b in bodies {
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let converted_str = str::from_utf8(&slice).unwrap();
                let mut dots_unfiltered_str = converted_str.to_string();
                // расписать случай если не найдет посты
                let mut count_for_items = 0;
                loop {
                    match dots_unfiltered_str.find("<dc:title>") {
                        Some(_) => match dots_unfiltered_str.find("</dc:title>") {
                            Some(_) => {
                                dots_unfiltered_str =
                                    dots_unfiltered_str.replace("<dc:title>", "<dcstitle>");
                                dots_unfiltered_str =
                                    dots_unfiltered_str.replace("</dc:title>", "</dcstitle>");
                                count_for_items += 1;
                            }
                            _ => {
                                break;
                            }
                        },
                        _ => {
                            break;
                        }
                    }
                }
                if count_for_items > 0 {
                    let biorvix_struct: XmlBiorxivParserStruct =
                        from_str(&dots_unfiltered_str).unwrap();
                    let mut count = 0;
                    let mut biorxiv_page_struct: BiorxivPageStruct = BiorxivPageStruct::new();
                    let mut xml_parser_one_string_creators =
                        biorvix_struct.items[count].creator.clone();
                    loop {
                        if count < biorvix_struct.items.len() {
                            biorxiv_page_struct.items[count].title =
                                biorvix_struct.items[count].title.clone();
                            biorxiv_page_struct.items[count].link =
                                biorvix_struct.items[count].link.clone();
                            biorxiv_page_struct.items[count].description =
                                biorvix_struct.items[count].description.clone();
                            match xml_parser_one_string_creators.find("., ") {
                                Some(end_of_creator) => {
                                    biorxiv_page_struct.items[count].creators.push(
                                        xml_parser_one_string_creators[..end_of_creator]
                                            .to_string(),
                                    );
                                    xml_parser_one_string_creators = xml_parser_one_string_creators
                                        [end_of_creator + "., ".len()..]
                                        .to_string();
                                }
                                None => {
                                    biorxiv_page_struct.items[count]
                                        .creators
                                        .push(xml_parser_one_string_creators.clone());
                                    break;
                                }
                            }
                            biorxiv_page_struct.items[count].date =
                                biorvix_struct.items[count].date.clone();
                            biorxiv_page_struct.items[count].publisher =
                                biorvix_struct.items[count].publisher.clone();
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    biorxiv_structs_vec
                        .insert(vec_of_keys[key_count].to_string(), biorxiv_page_struct);
                } else {
                    if ENABLE_PRINTS_BIORXIV {
                        println!("(biorxiv) no items for key {}", vec_of_keys[key_count]);
                    }
                    let useless_biorxiv_page_struct = BiorxivPageStruct::new();
                    biorxiv_structs_vec.insert(
                        vec_of_keys[key_count].to_string(),
                        useless_biorxiv_page_struct,
                    );
                }
            }
            Err(e) => {
                if ENABLE_ERROR_PRINTS_BIORXIV {
                    print_error_red(
                        file!().to_string(),
                        line!().to_string(),
                        e.to_string() + &"\nkey count: ".to_string() + &key_count.to_string(),
                    )
                }
            }
        }
        key_count += 1;
    }
    if ENABLE_PRINTS_BIORXIV {
        println!(
            "parsing in {} seconds, biorxiv bodies: {} ",
            time.elapsed().as_secs(),
            biorxiv_structs_vec.len()
        );
    }
    biorxiv_structs_vec.clone()
}
