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

use crate::get_group_names::get_medrxiv_links::get_medrxiv_links;

use crate::check_provider::can_i_reach_provider::reach_provider;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlBiorxivParserStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<XmlBiorxivParserStructItem>,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlBiorxivParserStructItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creator: String,
    pub date: String,
    pub publisher: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BiorxivPageStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<BiorxivPageStructItem>,
}

impl BiorxivPageStruct {
    pub fn new() -> Self {
        BiorxivPageStruct {
            // items: Vec::<BiorxivPageStructItem>::new(),
            items: vec![BiorxivPageStructItem::new(); 30],
            //vec![UsedRedditJsonStruct::new(); 25],
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BiorxivPageStructItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creators: Vec<String>,
    pub date: String,
    pub publisher: String,
}
impl BiorxivPageStructItem {
    pub fn new() -> Self {
        BiorxivPageStructItem {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            creators: Vec::<String>::new(),
            date: "".to_string(),
            publisher: "".to_string(),
        }
    }
}

#[tokio::main]
pub async fn fetch_and_parse_xml_biorxiv(
    vec_of_links: Vec<&str>,
    vec_of_keys: Vec<&str>,
) -> HashMap<String, BiorxivPageStruct> {
    let time = Instant::now();
    let mut biorxiv_structs_vec: HashMap<String, BiorxivPageStruct> =
        HashMap::with_capacity(vec_of_links.len());
    let client = Client::new();
    println!("starting fetching medrxiv...");
    let bodies = future::join_all(vec_of_links.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    }))
    .await;
    println!(
        "med future::join_all (in seconds) = {} ",
        time.elapsed().as_secs()
    );
    println!("medrxiv bodies.len() {}", bodies.len());
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
                    println!("(medarxiv) no items for key {}", vec_of_keys[key_count]);
                    let useless_biorxiv_page_struct = BiorxivPageStruct::new();
                    biorxiv_structs_vec.insert(
                        vec_of_keys[key_count].to_string(),
                        useless_biorxiv_page_struct,
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
    println!(
        "biorxiv xml parsing (in seconds) = {} ",
        time.elapsed().as_secs()
    );
    biorxiv_structs_vec.clone()
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
        let vec_of_vec_of_strings = fetch_and_parse_xml_biorxiv(vec_of_links, vec_of_keys);
        // vec_of_vec_of_strings //HashMap<String, BiorxivPageStruct>
        return true; //чекнуть действительно ли в векторе есть хоть шот полезное
                     // vec_of_vec_of_strings //еще надо подумать куда это записывать
    } else {
        return false;
    }
}
