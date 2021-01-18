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

use crate::get_group_names::get_arxiv_links::get_arxiv_links;

use crate::check_provider::can_i_reach_provider::reach_provider;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlArxivParserStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<XmlArxivParserPost>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlArxivParserPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creator: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivPostStruct {
    pub items: Vec<ArxivPost>,
}
//count: usize
impl ArxivPostStruct {
    pub fn new() -> Self {
        ArxivPostStruct {
            items: Vec::<ArxivPost>::new(),
            // items: vec![ArxivPost::new(); count],
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creators: Vec<Creator>,
}

impl ArxivPost {
    pub fn new() -> Self {
        ArxivPost {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            creators: Vec::<Creator>::new(),
            // creators: vec![Creator::new(); 70],
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Creator {
    pub name: String,
    pub link: String,
}

impl Creator {
    pub fn new() -> Self {
        Creator {
            name: "".to_string(),
            link: "".to_string(),
        }
    }
}

#[tokio::main]
pub async fn fetch_and_parse_xml_biorxiv(
    vec_of_links: Vec<&str>,
    vec_of_keys: Vec<&str>,
) -> HashMap<String, ArxivPostStruct> {
    let time = Instant::now();
    let mut biorxiv_structs_vec: HashMap<String, ArxivPostStruct> =
        HashMap::with_capacity(vec_of_links.len());
    let client = Client::new();
    println!("starting fetching arxiv...");
    let bodies = future::join_all(vec_of_links.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    }))
    .await;
    println!(
        "arxiv future::join_all (in seconds) = {} ",
        time.elapsed().as_secs()
    );
    println!("arxiv bodies.len() {}", bodies.len());
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
                        let biorvix_struct: XmlArxivParserStruct =
                            from_str(&dots_unfiltered_str).unwrap();
                        let mut count = 0;
                        let mut biorxiv_page_struct: ArxivPostStruct = ArxivPostStruct::new();
                        //biorvix_struct.items.len()
                        loop {
                            if count < biorvix_struct.items.len() {
                                let mut arxiv_post: ArxivPost = ArxivPost::new();
                                arxiv_post.title = biorvix_struct.items[count].title.clone();
                                arxiv_post.link = biorvix_struct.items[count].link.clone();
                                arxiv_post.description =
                                    biorvix_struct.items[count].description.clone();
                                let mut string_part_for_loop =
                                    biorvix_struct.items[count].creator.clone();
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
                                biorxiv_page_struct.items.push(arxiv_post);
                                count += 1;
                            } else {
                                break;
                            }
                        }
                        biorxiv_structs_vec
                            .insert(vec_of_keys[key_count].to_string(), biorxiv_page_struct);
                    }
                    _ => {
                        println!("(arxiv) no items for key {}", vec_of_keys[key_count]);
                        let useless_biorxiv_page_struct = ArxivPostStruct::new();
                        biorxiv_structs_vec.insert(
                            vec_of_keys[key_count].to_string(),
                            useless_biorxiv_page_struct,
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
    println!(
        "arxiv xml parsing (in seconds) = {} ",
        time.elapsed().as_secs()
    );
    biorxiv_structs_vec.clone()
}

pub fn arxiv_part(arxiv_url: &str) -> bool {
    if reach_provider(arxiv_url.to_string()) {
        let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links();
        println!(
            "{:#?} elements in Arxiv HashMap",
            arxiv_links_in_hash_map.len()
        );
        let vec_of_links: Vec<&str> = arxiv_links_in_hash_map.values().cloned().collect();
        let vec_of_keys: Vec<&str> = arxiv_links_in_hash_map.keys().cloned().collect();
        let vec_of_vec_of_strings = fetch_and_parse_xml_biorxiv(vec_of_links, vec_of_keys);
        return true; //чекнуть действительно ли в векторе есть хоть шот полезное
                     // vec_of_vec_of_strings //еще надо подумать куда это записывать//HashMap<String, ArxivPostStruct>
    } else {
        return false;
    }
}
