extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use futures::future;
use reqwest::Client;
use serde_xml_rs::from_str;
use std::collections::HashMap;
use std::str;
use std::time::Instant;
use tokio;

use crate::config::MEDRXIV_URL;
use crate::fetch::medrxiv_fetch::medrxiv_structures::MedrxivPageStruct;
use crate::fetch::medrxiv_fetch::medrxiv_structures::XmlMedrxivParserStruct;
use crate::get_group_names::get_medrxiv_links::get_medrxiv_links;
use crate::check_provider::can_i_reach_provider::reach_provider;

#[tokio::main]
pub async fn fetch_and_parse_xml_medrxiv(
    vec_of_links: Vec<&str>,
    vec_of_keys: Vec<&str>,
) -> HashMap<String, MedrxivPageStruct> {
    let time = Instant::now();
    let mut medrxiv_structs_vec: HashMap<String, MedrxivPageStruct> =
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
    println!("fetch in {} seconds, medrxiv bodies: {} ", time.elapsed().as_secs(), bodies.len());
    let mut key_count = 0;
    for b in bodies {
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let converted_str = str::from_utf8(&slice).unwrap();
                let mut dots_unfiltered_str = converted_str.to_string();
                dots_unfiltered_str.remove(0);//ОЧЕНЬ ВАЖНАЯ СТРОЧКА. НУЖНО УДАЛИТЬ ПЕРВУЮ ЧАСТЬ ЧТОБЫ ФАЙЛ ПРАВИЛЬНО СЧИТАЛСЯ
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
                    let medrvix_struct: XmlMedrxivParserStruct =
                        from_str(&dots_unfiltered_str).unwrap();
                    let mut count = 0;
                    let mut medrxiv_page_struct: MedrxivPageStruct = MedrxivPageStruct::new();
                    let mut xml_parser_one_string_creators =
                        medrvix_struct.items[count].creator.clone();
                    loop {
                        if count < medrvix_struct.items.len() {
                            medrxiv_page_struct.items[count].title =
                                medrvix_struct.items[count].title.clone();
                            medrxiv_page_struct.items[count].link =
                                medrvix_struct.items[count].link.clone();
                            medrxiv_page_struct.items[count].description =
                                medrvix_struct.items[count].description.clone();
                            match xml_parser_one_string_creators.find("., ") {
                                Some(end_of_creator) => {
                                    medrxiv_page_struct.items[count].creators.push(
                                        xml_parser_one_string_creators[..end_of_creator]
                                            .to_string(),
                                    );
                                    xml_parser_one_string_creators = xml_parser_one_string_creators
                                        [end_of_creator + "., ".len()..]
                                        .to_string();
                                }
                                None => {
                                    medrxiv_page_struct.items[count]
                                        .creators
                                        .push(xml_parser_one_string_creators.clone());
                                    break;
                                }
                            }
                            medrxiv_page_struct.items[count].date =
                                medrvix_struct.items[count].date.clone();
                            medrxiv_page_struct.items[count].publisher =
                                medrvix_struct.items[count].publisher.clone();
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    medrxiv_structs_vec
                        .insert(vec_of_keys[key_count].to_string(), medrxiv_page_struct);
                } else {
                    println!("(medarxiv) no items for key {}", vec_of_keys[key_count]);
                    let useless_medrxiv_page_struct = MedrxivPageStruct::new();
                    medrxiv_structs_vec.insert(
                        vec_of_keys[key_count].to_string(),
                        useless_medrxiv_page_struct,
                    );
                }
            }
            Err(e) => {
                println!("key_count on EERRRRRRRRRROOOOORRRRR {}", key_count);
                eprintln!("Got an error: {}", e);
            }
        }
        // println!("key_count {}", key_count);
        key_count += 1;
    }
    println!("parsing in {} seconds, medrxiv bodies: {} ", time.elapsed().as_secs(), medrxiv_structs_vec.len());
    medrxiv_structs_vec.clone()
}

pub fn medrxiv_part() -> bool {
    if reach_provider(MEDRXIV_URL.to_string()) {
        let arxiv_links_in_hash_map: HashMap<&str, &str> = get_medrxiv_links();
        println!(
            "{:#?} elements in Medrxiv HashMap",
            arxiv_links_in_hash_map.len()
        );
        let vec_of_links: Vec<&str> = arxiv_links_in_hash_map.values().cloned().collect();
        let vec_of_keys: Vec<&str> = arxiv_links_in_hash_map.keys().cloned().collect();
        fetch_and_parse_xml_medrxiv(vec_of_links, vec_of_keys);//тут есть возвращаемое значение let vec_of_vec_of_strings = 
        return true; //чекнуть действительно ли в векторе есть хоть шот полезное
    } else {
        return false;
    }
}
