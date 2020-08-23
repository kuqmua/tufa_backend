extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;
// extern crate xml;
use futures::future;
use reqwest::Client;
use serde_xml_rs::from_str;
use std::collections::HashMap;
use std::str;
use std::time::Instant;
use tokio;

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
    // pub creator: String,
    pub date: String,
    // pub publisher: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BiorxivPageStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<BiorxivPageStructItem>,
}

impl BiorxivPageStruct {
    pub fn new() -> Self {
        BiorxivPageStruct {
            items: Vec::<BiorxivPageStructItem>::new(),
            // items: vec![BiorxivPageStructItem::new(); 30],
            //vec![UsedRedditJsonStruct::new(); 25],
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BiorxivPageStructItem {
    pub title: String,
    pub link: String,
    pub description: String,
    // pub creators: Vec<String>,
    pub date: String,
    // pub publisher: String,
}
// impl BiorxivPageStructItem {
//     pub fn new() -> Self {
//         BiorxivPageStructItem {
//             title: "".to_string(),
//             link: "".to_string(),
//             description: "".to_string(),
//             // creators: Vec::<String>::new(),
//             date: "".to_string(),
//             // publisher: "".to_string(),
//         }
//     }
// }

#[tokio::main]
pub async fn fetch_and_parse_xml_biorxiv(
    vec_of_links: Vec<&str>,
    vec_of_keys: Vec<&str>,
) -> HashMap<String, BiorxivPageStruct> {
    let time = Instant::now();
    let mut biorxiv_structs_vec: HashMap<String, BiorxivPageStruct> =
        HashMap::with_capacity(vec_of_links.len());
    let client = Client::new();
    println!("starting fetching biorxiv...");
    let bodies = future::join_all(vec_of_links.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    }))
    .await;
    println!(
        "biorxiv future::join_all (in seconds) = {} ",
        time.elapsed().as_secs()
    );
    println!("biorxiv bodies.len() {}", bodies.len());
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
                    let xml_biorxiv_struct: XmlBiorxivParserStruct =
                        from_str(&dots_unfiltered_str).unwrap();
                    let mut count = 0;
                    let mut biorxiv_page_struct: BiorxivPageStruct = BiorxivPageStruct::new();
                    // let mut xml_parser_one_string_creators =
                    //     xml_biorxiv_struct.items[count].creator.clone();
                    loop {
                        if count < xml_biorxiv_struct.items.len() {
                            let temporary_title = xml_biorxiv_struct.items[count].title.clone();
                            let temporary_link = xml_biorxiv_struct.items[count].link.clone();
                            let temporary_description =
                                xml_biorxiv_struct.items[count].description.clone();
                            // let mut temporary_creators = Vec::new();
                            let temporary_date = xml_biorxiv_struct.items[count].date.clone();

                            // biorxiv_page_struct.items[count].title =
                            //     xml_biorxiv_struct.items[count].title.clone();
                            // biorxiv_page_struct.items[count].link =
                            //     xml_biorxiv_struct.items[count].link.clone();
                            // biorxiv_page_struct.items[count].description =
                            //     xml_biorxiv_struct.items[count].description.clone();
                            // biorxiv_page_struct.items[count].date =
                            //     xml_biorxiv_struct.items[count].date.clone();
                            /////////////////////////////
                            // match xml_parser_one_string_creators.find("., ") {
                            //     Some(end_of_creator) => {
                            //         biorxiv_page_struct.items[count].creators.push(
                            //             xml_parser_one_string_creators[..end_of_creator]
                            //                 .to_string(),
                            //         );
                            //         xml_parser_one_string_creators = xml_parser_one_string_creators
                            //             [end_of_creator + "., ".len()..]
                            //             .to_string();
                            //     }
                            //     None => {
                            //         biorxiv_page_struct.items[count]
                            //             .creators
                            //             .push(xml_parser_one_string_creators.clone());
                            //         break;
                            //     }
                            // }
                            // biorxiv_page_struct.items[count].publisher =
                            //     biorvix_struct.items[count].publisher.clone();
                            let temporary_biorxiv_page_struct_item: BiorxivPageStructItem =
                                BiorxivPageStructItem {
                                    title: temporary_title,
                                    link: temporary_link,
                                    description: temporary_description,
                                    // creators: temporary_creators,
                                    date: temporary_date,
                                    // publisher: temporary_publisher,
                                };
                            biorxiv_page_struct
                                .items
                                .push(temporary_biorxiv_page_struct_item);
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    biorxiv_structs_vec
                        .insert(vec_of_keys[key_count].to_string(), biorxiv_page_struct);
                } else {
                    println!("(biorxiv) no items for key {}", vec_of_keys[key_count]);
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

pub fn biorxiv_part() -> HashMap<String, BiorxivPageStruct> {
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links_in_hash_map();
    println!(
        "{:#?} elements in Biorxiv HashMap",
        arxiv_links_in_hash_map.len()
    );
    let vec_of_links: Vec<&str> = arxiv_links_in_hash_map.values().cloned().collect();
    let vec_of_keys: Vec<&str> = arxiv_links_in_hash_map.keys().cloned().collect();
    let vec_of_vec_of_strings = fetch_and_parse_xml_biorxiv(vec_of_links, vec_of_keys);
    vec_of_vec_of_strings
}

pub fn get_arxiv_links_in_hash_map() -> HashMap<&'static str, &'static str> {
    let arxiv_sections_links: HashMap<&str,&str> =
    [("Animal behavior and cognition","http://connect.biorxiv.org/biorxiv_xml.php?subject=animal_behavior_and_cognition"),
     ("Biochemistry","http://connect.biorxiv.org/biorxiv_xml.php?subject=biochemistry"),
     ("Bioengineering","http://connect.biorxiv.org/biorxiv_xml.php?subject=bioengineering"),
     ("Bioinformatics","http://connect.biorxiv.org/biorxiv_xml.php?subject=bioinformatics"),
     ("Biophysics","http://connect.biorxiv.org/biorxiv_xml.php?subject=biophysics"),
     ("Cancer biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=cancer_biology"),
     ("Cell biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=cell_biology"),
     ("Clinical trials","http://connect.biorxiv.org/biorxiv_xml.php?subject=clinical_trials"),
     ("Developmental biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=developmental_biology"),
     ("Ecology","http://connect.biorxiv.org/biorxiv_xml.php?subject=ecology"),
     ("Epidemology","http://connect.biorxiv.org/biorxiv_xml.php?subject=epidemiology"),
     ("Evolutionary biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=evolutionary_biology"),
     ("Genetics","http://connect.biorxiv.org/biorxiv_xml.php?subject=genetics"),
     ("Genomics","http://connect.biorxiv.org/biorxiv_xml.php?subject=genomics"),
     ("Immunology","http://connect.biorxiv.org/biorxiv_xml.php?subject=immunology"),
     ("Microbiology","http://connect.biorxiv.org/biorxiv_xml.php?subject=microbiology"),
     ("Molecular biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=molecular_biology"),
     ("Neuroscience","http://connect.biorxiv.org/biorxiv_xml.php?subject=neuroscience"),
    ("Paleontology","http://connect.biorxiv.org/biorxiv_xml.php?subject=paleontology"),
    ("Pathology","http://connect.biorxiv.org/biorxiv_xml.php?subject=pathology"),
    ("Pharmacology and toxicology","http://connect.biorxiv.org/biorxiv_xml.php?subject=pharmacology_and_toxicology"),
    ("Physiology","http://connect.biorxiv.org/biorxiv_xml.php?subject=physiology"),
    ("Plant Biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=plant_biology"),
    ("Scientific communication and education","http://connect.biorxiv.org/biorxiv_xml.php?subject=scientific_communication_and_education"),
    ("Synthetic Biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=synthetic_biology"),
    ("Systems Biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=systems_biology"),
    ("Zoology","http://connect.biorxiv.org/biorxiv_xml.php?subject=zoology"),
     ]
     .iter().cloned().collect();
    arxiv_sections_links
}
