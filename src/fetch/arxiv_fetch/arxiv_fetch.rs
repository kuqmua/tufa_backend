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

use crate::fetch::arxiv_fetch::arxiv_structures::Creator;
use crate::fetch::arxiv_fetch::arxiv_structures::ArxivPost;
use crate::fetch::arxiv_fetch::arxiv_structures::ArxivPostStruct;
use crate::fetch::arxiv_fetch::arxiv_structures::XmlArxivParserStruct;

use crate::config::ARXIV_URL;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;

use crate::check_provider::can_i_reach_provider::reach_provider;

#[tokio::main]
pub async fn fetch_and_parse_xml_arxiv(
    vec_of_links: Vec<&str>,
    vec_of_keys: Vec<&str>,
) -> HashMap<String, ArxivPostStruct> {
    let time = Instant::now();
    let mut arxiv_structs_vec: HashMap<String, ArxivPostStruct> =
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
    println!("fetch in {} seconds, arxiv bodies: {} ", time.elapsed().as_secs(), bodies.len());
    let mut key_count = 0;
    for b in bodies {
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let converted_str = str::from_utf8(&slice).unwrap();
                let dots_unfiltered_str = converted_str.to_string();
                // расписать случай если не найдет посты
                match dots_unfiltered_str.find("</item>") {
                    Some(_) => {
                        let arxiv_struct: XmlArxivParserStruct =
                            from_str(&dots_unfiltered_str).unwrap();
                        let mut count = 0;
                        let mut arxiv_page_struct: ArxivPostStruct = ArxivPostStruct::new();
                        //arxiv_struct.items.len()
                        loop {
                            if count < arxiv_struct.items.len() {
                                let mut arxiv_post: ArxivPost = ArxivPost::new();
                                arxiv_post.title = arxiv_struct.items[count].title.clone();
                                arxiv_post.link = arxiv_struct.items[count].link.clone();
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
                                                [link_index_from_start + "<a href=\"".len()
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
                        arxiv_structs_vec
                            .insert(vec_of_keys[key_count].to_string(), arxiv_page_struct);
                    }
                    _ => {
                        println!("(arxiv) no items for key {}", vec_of_keys[key_count]);
                        let useless_arxiv_page_struct = ArxivPostStruct::new();
                        arxiv_structs_vec.insert(
                            vec_of_keys[key_count].to_string(),
                            useless_arxiv_page_struct,
                        );
                    }
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
    println!("parsing in {} seconds, arxiv bodies: {} ", time.elapsed().as_secs(), arxiv_structs_vec.len());
    arxiv_structs_vec.clone()
}
pub fn arxiv_part() -> bool {
    if reach_provider(ARXIV_URL.to_string()) {
        let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links();
        println!(
            "{:#?} elements in Arxiv HashMap",
            arxiv_links_in_hash_map.len()
        );
        let vec_of_links: Vec<&str> = arxiv_links_in_hash_map.values().cloned().collect();
        let vec_of_keys: Vec<&str> = arxiv_links_in_hash_map.keys().cloned().collect();
        fetch_and_parse_xml_arxiv(vec_of_links, vec_of_keys);//тут есть возвращаемое значение let vec_of_vec_of_strings = 
        return true; //чекнуть действительно ли в векторе есть хоть шот полезное
    } else {
        return false;
    }
}
