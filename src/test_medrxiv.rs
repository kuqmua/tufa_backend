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
#[path = "./providers/initialization/check_providers_status/can_i_reach_provider.rs"]
mod can_i_reach_provider;

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
    if can_i_reach_provider::can_i_reach_provider("http://connect.medrxiv.org/".to_string()) {
        let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links_in_hash_map();
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

pub fn get_arxiv_links_in_hash_map() -> HashMap<&'static str, &'static str> {
    let arxiv_sections_links: HashMap<&str,&str> =
    [
     ("Addiction Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Addiction_Medicine"),
     ("Allergy and Immunology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Allergy_and_Immunology"),
     ("Anesthesia","http://connect.medrxiv.org/medrxiv_xml.php?subject=Anesthesia"),
     ("Cardiovascular Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Cardiovascular_Medicine"),
     ("Dentistry and Oral Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Dentistry_and_Oral_Medicine"),
     ("Dermatology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Dermatology"),
     ("Emergency Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Emergency_Medicine"),
     ("Endocrinology","http://connect.medrxiv.org/medrxiv_xml.php?subject=endocrinology"),
     ("Epidemiology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Epidemiology"),
     ("Forensic Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Forensic_Medicine"),
     ("Gastroenterology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Gastroenterology"),
     ("Genetic and Genomic Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Genetic_and_Genomic_Medicine"),
     ("Geriatric Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Geriatric_Medicine"),
     ("Health Economics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Health_Economics"),
     ("Health Informatics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Health_Informatics"),
     ("Health Policy","http://connect.medrxiv.org/medrxiv_xml.php?subject=Health_Policy"),
     ("Health Systems and Quality Improvement","http://connect.medrxiv.org/medrxiv_xml.php?subject=Health_Systems_and_Quality_Improvement"),
     ("Hematology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Hematology"),
     ("HIV/AIDS","http://connect.medrxiv.org/medrxiv_xml.php?subject=hivaids"),
     ("Infectious Diseases","http://connect.medrxiv.org/medrxiv_xml.php?subject=infectious_diseases"),
     ("Intensive Care and Critical Care Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Intensive_Care_and_Critical_Care_Medicine"),
     ("Medical Education","http://connect.medrxiv.org/medrxiv_xml.php?subject=Medical_Education"),
     ("Medical Ethics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Medical_Ethics"),
     ("Nephrology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Nephrology"),
     ("Neurology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Neurology"),
     ("Nursing","http://connect.medrxiv.org/medrxiv_xml.php?subject=Nursing"),
     ("Nutrition","http://connect.medrxiv.org/medrxiv_xml.php?subject=Nutrition"),
     ("Obstetrics and Gynecology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Obstetrics_and_Gynecology"),
     ("Occupational and Environmental Health","http://connect.medrxiv.org/medrxiv_xml.php?subject=Occupational_and_Environmental_Health"),
     ("Oncology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Oncology"),
     ("Ophthalmology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Ophthalmology"),
     ("Orthopedics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Orthopedics"),
     ("Otolaryngology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Otolaryngology"),
     ("Pain Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Pain_Medicine"),
     ("Palliative Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Palliative_Medicine"),
     ("Pathology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Pathology"),
     ("Pediatrics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Pediatrics"),
     ("Pharmacology and Therapeutics","http://connect.medrxiv.org/medrxiv_xml.php?subject=Pharmacology_and_Therapeutics"),
     ("Primary Care Research","http://connect.medrxiv.org/medrxiv_xml.php?subject=Primary_Care_Research"),
     ("Psychiatry and Clinical Psychology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Psychiatry_and_Clinical_Psychology"),
     ("Public and Global Health","http://connect.medrxiv.org/medrxiv_xml.php?subject=Public_and_Global_Health"),
     ("Radiology and Imaging","http://connect.medrxiv.org/medrxiv_xml.php?subject=Radiology_and_Imaging"),
     ("Rehabilitation Medicine and Physical Therapy","http://connect.medrxiv.org/medrxiv_xml.php?subject=Rehabilitation_Medicine_and_Physical_Therapy"),
     ("Respiratory Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Respiratory_Medicine"),
     ("Rheumatology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Rheumatology"),
     ("Sexual and Reproductive Health","http://connect.medrxiv.org/medrxiv_xml.php?subject=Sexual_and_Reproductive_Health"),
     ("Sports Medicine","http://connect.medrxiv.org/medrxiv_xml.php?subject=Sports_Medicine"),
     ("Surgery","http://connect.medrxiv.org/medrxiv_xml.php?subject=Surgery"),
     ("Toxicology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Toxicology"),
     ("Transplantation","http://connect.medrxiv.org/medrxiv_xml.php?subject=Transplantation"),
     ("Urology","http://connect.medrxiv.org/medrxiv_xml.php?subject=Urology"),
     ]
     .iter().cloned().collect();
    arxiv_sections_links
}
