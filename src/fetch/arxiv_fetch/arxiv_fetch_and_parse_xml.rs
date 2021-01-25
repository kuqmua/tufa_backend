use futures::future;
use reqwest::Client;
use serde_xml_rs::from_str;
use std::collections::HashMap;
use std::str;
use std::thread;
use std::time::Instant;
use tokio;
use xml::Element;
use xml::Xml;

use super::arxiv_structures::ArxivPost;
use super::arxiv_structures::ArxivPostStruct;
use super::arxiv_structures::Creator;
use super::arxiv_structures::XmlArxivParserStruct;
use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::override_prints::override_prints::print_error_red;

#[tokio::main]
pub async fn arxiv_fetch_and_parse_xml(
    //&'a <'a>
    vec_of_links: Vec<&str>,
    vec_of_keys: Vec<&str>,
) -> HashMap<String, ArxivPostStruct> {
    let time = Instant::now();
    let client = Client::new();
    let vec_links_len = vec_of_links.len();
    let bodies = future::join_all(vec_of_links.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    }))
    .await;
    let mut arxiv_structs_vec: HashMap<String, ArxivPostStruct> =
        HashMap::with_capacity(vec_links_len);
    if ENABLE_PRINTS_ARXIV {
        println!(
            "fetch in {} seconds, arxiv bodies: {} ",
            time.elapsed().as_secs(),
            bodies.len()
        );
    }
    let mut key_count = 0;
    // let mut threads_vec = vec![];
    for b in bodies {
        // threads_vec.push(thread::spawn(move || {
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let converted_str = str::from_utf8(&slice).unwrap();
                let dots_unfiltered_str = converted_str.to_string();
                let elem: std::result::Result<Element, _> = dots_unfiltered_str.parse();
                // if ENABLE_PRINTS_ARXIV {
                //     println!(
                //         "parse in {} seconds, in {} miliseconds",
                //         time.elapsed().as_secs(),
                //         time.elapsed().as_millis()
                //     );
                // }
                match elem {
                    Ok(e) => {
                        for r in e.children {
                            match r {
                                Xml::ElementNode(value) => {
                                    if value.name == "item" {
                                        let mut arxiv_post: ArxivPost = ArxivPost::new();
                                        for w in value.children {
                                            match w {
                                                Xml::ElementNode(valueel) => {
                                                    if valueel.children.len() != 0 {
                                                        for m in valueel.children {
                                                            match m {
                                                                Xml::CharacterNode(val) => {
                                                                    if valueel.name == "title" {
                                                                        if let Some(
                                                                            title_end_index,
                                                                        ) = val.find(". (")
                                                                        {
                                                                            let titleee = &val
                                                                                [..title_end_index];
                                                                            // println!(
                                                                            //     "title2, {}",
                                                                            //     titleee
                                                                            // );
                                                                            arxiv_post.title =
                                                                                titleee.to_string();
                                                                        } else {
                                                                            println!("fak");
                                                                        }
                                                                    }
                                                                    if valueel.name == "link" {
                                                                        // println!("link, {}", val);
                                                                        arxiv_post.title =
                                                                            val.clone();
                                                                    }
                                                                    if valueel.name == "description"
                                                                    {
                                                                        if let Some(
                                                                            description_start_index,
                                                                        ) = val.find("<p>")
                                                                        {
                                                                            if let Some(
                                                                                description_end_index,
                                                                            ) = val.find("</p>")
                                                                            {
                                                                                let description = &val
                                                                                [description_start_index + "<p>".len()..description_end_index];
                                                                                // println!(
                                                                                //     "description, {}",
                                                                                //     description
                                                                                // );
                                                                                arxiv_post
                                                                                    .description =
                                                                                    description
                                                                                        .to_string(
                                                                                        );
                                                                            }
                                                                        } else {
                                                                            println!("fak");
                                                                        }
                                                                    }
                                                                    if valueel.name == "creator" {
                                                                        let mut val_cloned =
                                                                            val.clone();
                                                                        //тут где то может упасть
                                                                        while let Some(
                                                                            link_index_from_start,
                                                                        ) = val_cloned
                                                                            .find("<a href=\"")
                                                                        {
                                                                            if let Some(
                                                                                link_index_from_end,
                                                                            ) = val_cloned
                                                                                .find("\">")
                                                                            {
                                                                                if let Some(
                                                                                    name_index_from_end,
                                                                                ) = val_cloned
                                                                                    .find("</a>")
                                                                                {
                                                                                    let mut creator =
                                                                                        Creator::new(
                                                                                        );
                                                                                    creator.link = val_cloned
                                                [link_index_from_start + "<a href=\"".len()
                                                    ..link_index_from_end]
                                                .to_string();
                                                                                    let name_index_from_start =
                                                link_index_from_end + "\">".len();
                                                                                    creator.name = val_cloned
                                                [name_index_from_start..name_index_from_end]
                                                .to_string();
                                                                                    val_cloned = val_cloned
                                                [name_index_from_end + "\">".len()..]
                                                .to_string();
                                                                                    // println!(
                                                                                    //     "fff {:#?}",
                                                                                    //     creator
                                                                                    // );
                                                                                    arxiv_post
                                                                                        .creators
                                                                                        .push(
                                                                                            creator,
                                                                                        );
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    // println!("f");
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    // println!("f");
                                                }
                                            }

                                            // z += 1;
                                        }
                                    }
                                }
                                _ => {
                                    // println!("f");
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if ENABLE_PRINTS_ARXIV {
                            println!("fff");
                        }
                    }
                }
                // if ENABLE_PRINTS_ARXIV {
                //     println!("fff {:#?}", elem); //dots_unfiltered_str
                // }

                // расписать случай если не найдет посты
                // match dots_unfiltered_str.find("</item>") {
                //     Some(_) => {
                //         let arxiv_struct: XmlArxivParserStruct =
                //             from_str(&dots_unfiltered_str).unwrap();
                //         let mut count = 0;
                //         let mut arxiv_page_struct: ArxivPostStruct = ArxivPostStruct::new();
                //         loop {
                //             if count < arxiv_struct.items.len() {
                //                 let mut arxiv_post: ArxivPost = ArxivPost::new();
                //                 arxiv_post.title = arxiv_struct.items[count].title.clone();
                //                 arxiv_post.link = arxiv_struct.items[count].link.clone();
                //                 arxiv_post.description =
                //                     arxiv_struct.items[count].description.clone();
                //                 let mut string_part_for_loop =
                //                     arxiv_struct.items[count].creator.clone();
                //                 while let Some(link_index_from_start) =
                //                     string_part_for_loop.find("<a href=\"")
                //                 {
                //                     if let Some(link_index_from_end) =
                //                         string_part_for_loop.find("\">")
                //                     {
                //                         if let Some(name_index_from_end) =
                //                             string_part_for_loop.find("</a>")
                //                         {
                //                             let mut creator = Creator::new();
                //                             creator.link = string_part_for_loop
                //                                 [link_index_from_start + "<a href=\"".len()
                //                                     ..link_index_from_end]
                //                                 .to_string();
                //                             let name_index_from_start =
                //                                 link_index_from_end + "\">".len();
                //                             creator.name = string_part_for_loop
                //                                 [name_index_from_start..name_index_from_end]
                //                                 .to_string();
                //                             string_part_for_loop = string_part_for_loop
                //                                 [name_index_from_end + "\">".len()..]
                //                                 .to_string();
                //                             arxiv_post.creators.push(creator);
                //                         }
                //                     }
                //                 }
                //                 arxiv_page_struct.items.push(arxiv_post);
                //                 count += 1;
                //             } else {
                //                 break;
                //             }
                //         }
                //         // arxiv_structs_vec
                //         //     .insert(vec_of_keys[key_count].to_string(), arxiv_page_struct);
                //     }
                //     _ => {
                //         if ENABLE_PRINTS_ARXIV {
                //             println!("(arxiv) no items for key {}", vec_of_keys[key_count]);
                //         }
                //         let useless_arxiv_page_struct = ArxivPostStruct::new();
                //         arxiv_structs_vec.insert(
                //             vec_of_keys[key_count].to_string(),
                //             useless_arxiv_page_struct,
                //         );
                //     }
                // }
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
        // }));
    }
    // for i in threads_vec {
    //     i.join().unwrap();
    // }
    if ENABLE_PRINTS_ARXIV {
        println!(
            "parsing in {} seconds, arxiv bodies: {} ",
            time.elapsed().as_secs(),
            arxiv_structs_vec.len()
        );
    }
    arxiv_structs_vec.clone()
}
