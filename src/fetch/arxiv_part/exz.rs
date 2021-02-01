use futures::future;
use reqwest::Client;
use serde_xml_rs::from_str;
use std::collections::HashMap;
use std::str;
use std::time::Instant;
use tokio;

use super::arxiv_structures::ArxivPost;
use super::arxiv_structures::ArxivPostStruct;
use super::arxiv_structures::Creator;
use super::arxiv_structures::XmlArxivParserStruct;
use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::override_prints::override_prints::print_error_red;

#[tokio::main]
pub async fn arxiv_fetch_and_parse_xml(
    vec_of_links: Vec<&str>,
    vec_of_keys: Vec<&str>,
) -> HashMap<String, ArxivPostStruct> {
    let time = Instant::now();

    let client = Client::new();
    let bodies = future::join_all(vec_of_links.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    }))
    .await;
    if ENABLE_PRINTS_ARXIV {
        println!(
            "fetch in {} seconds, arxiv bodies: {} ",
            time.elapsed().as_secs(),
            bodies.len()
        );
    }
    let mut key_count = 0;
    let mut arxiv_structs_vec: HashMap<String, ArxivPostStruct> =
        HashMap::with_capacity(vec_of_links.len());
    for b in bodies {
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let converted_str = str::from_utf8(&slice).unwrap();
            }
            Err(e) => {
                if ENABLE_ERROR_PRINTS_ARXIV {
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
    if ENABLE_PRINTS_ARXIV {
        println!(
            "parsing in {} seconds, arxiv bodies: {} ",
            time.elapsed().as_secs(),
            arxiv_structs_vec.len()
        );
    }
    arxiv_structs_vec.clone()
}

pub fn arxiv_parse_xml(converted_str: &str) {
    let dots_unfiltered_str = converted_str.to_string();
    // расписать случай если не найдет посты
    match dots_unfiltered_str.find("</item>") {
        Some(_) => {
            let arxiv_struct: XmlArxivParserStruct = from_str(&dots_unfiltered_str).unwrap();
            let mut count = 0;
            let mut arxiv_page_struct: ArxivPostStruct = ArxivPostStruct::new();
            loop {
                if count < arxiv_struct.items.len() {
                    let mut arxiv_post: ArxivPost = ArxivPost::new();
                    arxiv_post.title = arxiv_struct.items[count].title.clone();
                    arxiv_post.link = arxiv_struct.items[count].link.clone();
                    arxiv_post.description = arxiv_struct.items[count].description.clone();
                    let mut string_part_for_loop = arxiv_struct.items[count].creator.clone();
                    while let Some(link_index_from_start) = string_part_for_loop.find("<a href=\"")
                    {
                        if let Some(link_index_from_end) = string_part_for_loop.find("\">") {
                            if let Some(name_index_from_end) = string_part_for_loop.find("</a>") {
                                let mut creator = Creator::new();
                                creator.link = string_part_for_loop[link_index_from_start
                                    + "<a href=\"".len()
                                    ..link_index_from_end]
                                    .to_string();
                                let name_index_from_start = link_index_from_end + "\">".len();
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
            arxiv_structs_vec.insert(vec_of_keys[key_count].to_string(), arxiv_page_struct);
        }
        _ => {
            if ENABLE_PRINTS_ARXIV {
                println!("(arxiv) no items for key {}", vec_of_keys[key_count]);
            }
            let useless_arxiv_page_struct = ArxivPostStruct::new();
            arxiv_structs_vec.insert(
                vec_of_keys[key_count].to_string(),
                useless_arxiv_page_struct,
            );
        }
    }
}
