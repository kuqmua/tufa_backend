use futures::future;
use reqwest::{Client, Response};
use serde_xml_rs::from_str;
use std::collections::HashMap;
use std::str;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;
use tokio;
// use xml::Element;
// use xml::Xml;

use super::arxiv_structures::ArxivPost;
use super::arxiv_structures::ArxivPostStruct;
use super::arxiv_structures::Creator;
use super::arxiv_structures::XmlArxivParserStruct;
use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::override_prints::override_prints::print_error_red;

#[tokio::main]
pub async fn arxiv_fetch_and_parse_xml() -> HashMap<String, ArxivPostStruct> {
    let time = Instant::now();
    let client = Client::new();
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links();
    let vec_of_links: Vec<&str> = arxiv_links_in_hash_map.values().cloned().collect();
    let vec_of_keys: Vec<&str> = arxiv_links_in_hash_map.keys().cloned().collect();
    let vec_links_len = vec_of_links.len();
    let arcmap: HashMap<String, ArxivPostStruct> = HashMap::with_capacity(vec_links_len);
    println!("1");
    let arcmap = Arc::new(Mutex::new(arcmap));
    {
        let arcmap = arcmap.clone();
        tokio::spawn(async move {
            let resp = client.get(vec_of_links[0]).send().await;
            match resp {
                Ok(respok) => {
                    println!("{:#?}", respok.status() == reqwest::StatusCode::OK);
                    println!("fetch in {} seconds", time.elapsed().as_secs());
                    let g = respok.text().await;
                    match g {
                        Ok(text) => {
                            println!("fetch in {} seconds", time.elapsed().as_secs());
                            match text.find("</item>") {
                                Some(_) => {
                                    let arxiv_struct: XmlArxivParserStruct =
                                        from_str(&text).unwrap();
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
                                    arcmap
                                        .lock()
                                        .unwrap()
                                        .insert(vec_of_keys[0].to_string(), arxiv_page_struct);
                                }
                                _ => {
                                    if ENABLE_PRINTS_ARXIV {
                                        println!("(arxiv) no items for key {}", vec_of_keys[0]);
                                    }
                                    let useless_arxiv_page_struct = ArxivPostStruct::new();
                                    arcmap.lock().unwrap().insert(
                                        vec_of_keys[0].to_string(),
                                        useless_arxiv_page_struct,
                                    );
                                }
                            }
                        }
                        Err(e) => println!("{}", e),
                    }
                }
                Err(e) => println!("{}", e),
            }
        });
    };
    // handle.await.unwrap();
    println!("2");
    // let s = Arc::try_unwrap(arcmap);
    // let mut arcmapt: HashMap<String, ArxivPostStruct> = HashMap::new();
    // println!("3");
    // match s {
    //     Ok(oki) => {
    //         println!("yiiii");
    //         let f = oki.into_inner();
    //         match f {
    //             Ok(okidoki) => {
    //                 println!("yiiii2");
    //                 arcmapt = okidoki;
    //             }
    //             Err(e) => eprint!("ffff{:#?}", e),
    //         }
    //     }
    //     Err(e) => {
    //         s.f();
    //         eprint!("f2{:#?}", e)
    //     }
    // }
    println!("4");
    //Mutex<HashMap<std::string::String, ArxivPostStruct>>
    let arcmapt: HashMap<String, ArxivPostStruct> =
        Arc::try_unwrap(arcmap).unwrap().into_inner().unwrap();
    arcmapt
}
