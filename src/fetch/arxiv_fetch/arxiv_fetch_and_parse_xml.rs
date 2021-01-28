use reqwest;
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
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::override_prints::override_prints::print_error_red;

// pub async fn do_somethingh() -> HashMap<String, ArxivPostStruct>{
//     tokio::spawn(async move {
//         resp
//         match resp {
//             Ok(respok) => {
//                 println!("{:#?}", respok.status() == reqwest::StatusCode::OK);
//                 println!("fetch in {} seconds", time.elapsed().as_secs());
//                 let g = respok.text().await;
//                 match g {
//                     Ok(text) => {
//                         println!("fetch in {} seconds", time.elapsed().as_secs());
//                         match text.find("</item>") {
//                             Some(_) => {
//                                 let arxiv_struct: XmlArxivParserStruct = from_str(&text).unwrap();
//                                 let mut count = 0;
//                                 let mut arxiv_page_struct: ArxivPostStruct = ArxivPostStruct::new();
//                                 loop {
//                                     if count < arxiv_struct.items.len() {
//                                         let mut arxiv_post: ArxivPost = ArxivPost::new();
//                                         arxiv_post.title = arxiv_struct.items[count].title.clone();
//                                         arxiv_post.link = arxiv_struct.items[count].link.clone();
//                                         arxiv_post.description =
//                                             arxiv_struct.items[count].description.clone();
//                                         let mut string_part_for_loop =
//                                             arxiv_struct.items[count].creator.clone();
//                                         while let Some(link_index_from_start) =
//                                             string_part_for_loop.find("<a href=\"")
//                                         {
//                                             if let Some(link_index_from_end) =
//                                                 string_part_for_loop.find("\">")
//                                             {
//                                                 if let Some(name_index_from_end) =
//                                                     string_part_for_loop.find("</a>")
//                                                 {
//                                                     let mut creator = Creator::new();
//                                                     creator.link = string_part_for_loop
//                                                         [link_index_from_start + "<a href=\"".len()
//                                                             ..link_index_from_end]
//                                                         .to_string();
//                                                     let name_index_from_start =
//                                                         link_index_from_end + "\">".len();
//                                                     creator.name = string_part_for_loop
//                                                         [name_index_from_start
//                                                             ..name_index_from_end]
//                                                         .to_string();
//                                                     string_part_for_loop = string_part_for_loop
//                                                         [name_index_from_end + "\">".len()..]
//                                                         .to_string();
//                                                     arxiv_post.creators.push(creator);
//                                                 }
//                                             }
//                                         }

//                                         arxiv_page_struct.items.push(arxiv_post);
//                                         count += 1;
//                                     } else {
//                                         break;
//                                     }
//                                 }
//                                 arcmapt
//                                     .lock()
//                                     .unwrap()
//                                     .insert(vec_of_keys[0].to_string(), arxiv_page_struct);
//                             }
//                             _ => {
//                                 if ENABLE_PRINTS_ARXIV {
//                                     println!("(arxiv) no items for key {}", vec_of_keys[0]);
//                                 }
//                                 let useless_arxiv_page_struct = ArxivPostStruct::new();
//                                 arcmapt
//                                     .lock()
//                                     .unwrap()
//                                     .insert(vec_of_keys[0].to_string(), useless_arxiv_page_struct);
//                             }
//                         }
//                     }
//                     Err(e) => println!("{}", e),
//                 }
//             }
//             Err(e) => println!("{}", e),
//         }
//     });
//     arcmap
// }

// let mut arxiv_sections_links: HashMap<String, String> = [
//         (
//             "http://export.arxiv.org/rss/astro-ph.CO".to_string(),
//             "1".to_string(),
//         ),
//         (
//             "http://export.arxiv.org/rss/astro-ph.EP".to_string(),
//             "2".to_string(),
//         ),
//         (
//             "http://export.arxiv.org/rss/astro-ph.GA".to_string(),
//             "3".to_string(),
//         ),
//     ]
//     .iter()
//     .cloned()
//     .collect();
pub fn do_something() -> HashMap<String, (String, ArxivPostStruct)> {
    let time = Instant::now();
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links();
    let mut hashmap_to_return: HashMap<String, (String, ArxivPostStruct)> = HashMap::new();
    for (key, value) in arxiv_links_in_hash_map {
        let useless_arxiv_page_struct = ArxivPostStruct::new();
        let tuple = (value.to_string(), useless_arxiv_page_struct);
        hashmap_to_return.insert(key.to_string(), tuple);
    }

    // let mut h = 0;
    let crossbeam_result = crossbeam::scope(|scope| {
        for (key, value) in &mut hashmap_to_return {
            scope.spawn(move |_| {
                let fetch_result = fetch_link(&value.0, key);
                match fetch_result {
                    Ok(fetch_string_result) => {
                        //add better error handling
                        if fetch_string_result == "res.text error" {
                            println!("F for"); //, value.0
                        } else if fetch_string_result == "fetch_link status: NOT OK " {
                            println!("F2 for"); //, value.0
                        } else {
                            println!(
                                "fetch for {} done in {} seconds. starting parsing...",
                                key,
                                time.elapsed().as_secs()
                            );
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
                                        println!("(arxiv) no items for key {}", key);
                                    }
                                }
                            }
                            println!(
                                "ended parsing {} in {} seconds",
                                key,
                                time.elapsed().as_secs()
                            );
                        }
                    }
                    Err(e) => println!(" fetch_result error {}", e),
                }
                // *value = b;
            });
            // h += 1;
        }
    });
    // println!("arxiv_sections_links {:#?}", hashmap_to_return);
    hashmap_to_return
}

fn fetch_link(link: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    //add link identifire here for better user exp
    println!("start fetching for key: {}", key);
    let res = reqwest::blocking::get(link)?;
    println!("fetch_link status: {} for key: {}", res.status(), key);
    let b: String;
    if res.status() == reqwest::StatusCode::OK {
        let s = res.text();
        match s {
            Ok(norm) => {
                b = norm;
            }
            Err(e) => {
                println!("err{}", e);
                b = "res.text error".to_string();
            }
        }
    } else {
        b = "fetch_link status: NOT OK ".to_string();
    }
    Ok(b)
}
