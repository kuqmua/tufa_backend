use std::collections::HashMap;
extern crate reqwest;
extern crate xml;
use std::time::Instant;
use std::fmt::Display;
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::Client;
use futures::future;
use tokio;
use std::str;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creators: Vec<Creator>,
}

impl Display for ArxivPost {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "title = {}\nlink = {}\ndescription ={}\ncreators = {:#?}\n",
            self.title, self.link, self.description, self.creators
        )
    }
}

impl ArxivPost {
    pub fn new() -> Self {
        ArxivPost {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            creators: Vec::<Creator>::new(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Creator {
    pub name: String,
    pub link: String,
}

impl Display for Creator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "name = {}\nlink = {}\n", self.name, self.link)
    }
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
pub async fn async_test_function(vec_of_links: Vec<&str>, vec_of_keys: Vec<&str>,) -> HashMap<String, Vec<ArxivPost>>{
    let time = Instant::now();
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
    println!("biorxiv bodies.len() {}", bodies.len());
    println!(
            "future::join_all (in seconds) = {} ",
            time.elapsed().as_secs()
        );
    let mut count = 0;
    let mut vec_of_vec_of_strings: Vec<Vec<String>> = Vec::new();
    for b in bodies {
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let converted_string = str::from_utf8(&slice).unwrap();
                let mut reader = Reader::from_str(&converted_string);
                reader.trim_text(true);//может из-за этого не парсит?
                let mut vec_of_arxiv_rss_strings_what_contains_xmls = Vec::new();
                let mut buf = Vec::new();
                println!("biorxiv 92str count {}", count);
                loop {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Start(ref e)) => match e.name() {
                            b"tag1" => println!(
                                "attributes values: {:?}",
                                e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                            ),
                        _ => (),
                        },
                        Ok(Event::Text(e)) => vec_of_arxiv_rss_strings_what_contains_xmls.push(e.unescape_and_decode(&reader).unwrap()),
                        Ok(Event::Eof) => break,
                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                        _ => (),
                    }
                    buf.clear();
                }
                println!("biorxiv 109str count {}", count);
                let vec_of_string_removed_meta = remove_meta_elements_from_vec_of_arxiv_rss(vec_of_arxiv_rss_strings_what_contains_xmls);
                vec_of_vec_of_strings.push(vec_of_string_removed_meta);
                println!("biorxiv 112str count {}", count);
                count += 1;
            }
            Err(e) => {
                count += 1;
                eprintln!("Got an error: {}", e);
            }
        }
    }
    println!("count = {}", count);
    let mut arxiv_pages_posts_hashmap_second  = HashMap::new();
    let mut key_count = 0;
    for vec_member in vec_of_vec_of_strings {
        for i in vec_member.clone(){
        println!("{}", i);
    }
        let vec_of_arxiv_post_second: Vec<ArxivPost> = parse_arxiv_string_xml(vec_member, vec_of_keys[key_count].to_string());
        
        if vec_of_arxiv_post_second.len() != 0{
            if vec_of_arxiv_post_second[0].creators.len() != 0 {
           arxiv_pages_posts_hashmap_second.insert(vec_of_keys[key_count].to_string(), vec_of_arxiv_post_second);
        }
        else{
            println!("vec_of_arxiv_posts[0].creators.len() == 0");
        }
            }
        key_count +=1;
    }
    // println!("arxiv_pages_posts_hashmap_second done and contains {:#?} elements", arxiv_pages_posts_hashmap_second.len());
    // println!("Artificial Intelligence contains {:#?} elements", arxiv_pages_posts_hashmap_second["Artificial Intelligence"].len());
    println!("arxiv fetching/parsing done in {} seconds",time.elapsed().as_secs());
    arxiv_pages_posts_hashmap_second
}

pub fn test_biorxiv() ->  HashMap<String, Vec<ArxivPost>>
{
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links_in_hash_map();
    println!("{:#?}", arxiv_links_in_hash_map.len());
    let vec_of_links: Vec<&str> = arxiv_links_in_hash_map.values().cloned().collect();
    let vec_of_keys: Vec<&str> = arxiv_links_in_hash_map.keys().cloned().collect();
    let vec_of_vec_of_strings = async_test_function(vec_of_links, vec_of_keys);
    vec_of_vec_of_strings
}

fn remove_meta_elements_from_vec_of_arxiv_rss(mut vec_of_arxiv_rss: Vec<String>)->Vec<String>{
    println!("start remove_meta_elements_from_vec_of_arxiv_rss");
    let mut remove_element = 0;
    let mut shit = 0;
    while shit < vec_of_arxiv_rss.len(){
        println!("vec_of_arxiv_rss[{}] {} ", shit,  vec_of_arxiv_rss[shit]);
        shit +=1;
    }
    
    while remove_element < 13 {
        println!("trying to remove {} element", remove_element);
        vec_of_arxiv_rss.remove(0);
        remove_element += 1;
    }
    println!("end remove_meta_elements_from_vec_of_arxiv_rss");
    vec_of_arxiv_rss
}

pub fn parse_arxiv_string_xml(vec_of_arxiv_xml_string: Vec<String>, key: String)->Vec<ArxivPost>{
    
    let mut vec_of_arxiv_posts: Vec<ArxivPost> = Vec::new();
    let mut posts_count = 0;
    let end_index_title_string = ". (arXiv";
    let start_index_description_string = "<p>";
    let end_index_description_string = "\n</p>";
    let start_index_creator_link_string = "<a href=\"";
    let end_index_creator_link_string = "\">";
    let end_index_creator_name_string = "</a>";
    let starting_useless_symbols = "<![CDATA[";
    let ending_useless_symbols = "]]>";
    while posts_count < vec_of_arxiv_xml_string.len() {
        let mut arxiv_post: ArxivPost = ArxivPost::new();
        let title = vec_of_arxiv_xml_string[posts_count].clone();
        if let Some(start_title_index) = title.find("<![CDATA[") {
            if let Some(end_title_index) = title.find("]]>"){
                arxiv_post.title = title[start_title_index + starting_useless_symbols.len()..end_title_index].to_string();
            }
            else{
                arxiv_post.title = format!("Error in \"let title = vec_of_arxiv_xml_string[posts_count].clone();\".\n Find \"<![CDATA[\", but can not find \"]]>\"\n posts_count = {}, key = {}", posts_count, key);
                println!("Error title written\n posts_count = {}, key = {} \n Error in \"let title = vec_of_arxiv_xml_string[posts_count].clone();\".\n Can not \"<![CDATA[\" and \"]]>\"\n", posts_count, key);
            }   
        } else {
            arxiv_post.title = format!("Error in \"let title = vec_of_arxiv_xml_string[posts_count].clone();\".\n Can not \"<![CDATA[\" and \"]]>\"\n  posts_count = {}, key = {}", posts_count, key);
            println!("Error title written\n posts_count = {}, key = {} \n Error in \"let title = vec_of_arxiv_xml_string[posts_count].clone();\".\n Can not \"<![CDATA[\" and \"]]>\"\n", posts_count, key);
        }
        arxiv_post.link = vec_of_arxiv_xml_string[posts_count + 1].clone();
        let description = vec_of_arxiv_xml_string[posts_count + 2].clone();
        if let Some(start_description_index) = description.find("<![CDATA[") {
            if let Some(end_description_index) = description.find("]]>") {
                arxiv_post.description =
                    description[start_description_index + starting_useless_symbols.len()..end_description_index].to_string();
            } else {
                arxiv_post.description = format!("Error in \"let description = vec_of_arxiv_xml_string[posts_count + 2].clone();\".\n Find \"<![CDATA[\", but can not find \"]]>\"\n posts_count = {}, key = {}", posts_count, key);
                println!("Error title written\n posts_count = {}, key = {} \n Error in \"let description = vec_of_arxiv_xml_string[posts_count + 2].clone();\".\n Can not \"<![CDATA[\" and \"]]>\"\n", posts_count, key);
            }
        } else {
            arxiv_post.description  = format!("Error in \"let description = vec_of_arxiv_xml_string[posts_count + 2].clone();\".\n Can not \"<![CDATA[\" and \"]]>\"\n  posts_count = {}, key = {}", posts_count, key);
            println!("Error title written\n posts_count = {}, key = {} \n Error in \"let description = vec_of_arxiv_xml_string[posts_count + 2].clone();\".\n Can not \"<![CDATA[\" and \"]]>\"\n", posts_count, key);
        }
        let creators_string = vec_of_arxiv_xml_string[posts_count + 3].clone();
        let mut string_part_for_loop = creators_string;
        let mut creators_count = 0;
        while let Some(link_index_from_start) = string_part_for_loop.find("<a href=\"") {
            if creators_count > 256 {
                panic!("why creators_count > 256?"); //переписать когда примешь стандарт по которому ошибки обрабатывать будешь
            } else {
                if let Some(link_index_from_end) = string_part_for_loop.find(end_index_creator_link_string) {
                    if let Some(name_index_from_end) = string_part_for_loop.find(end_index_creator_name_string) {
                        let mut creator = Creator::new();
                        creator.link = string_part_for_loop[link_index_from_start + start_index_creator_link_string.len()..link_index_from_end].to_string();
                        let name_index_from_start = link_index_from_end + end_index_creator_link_string.len();
                        creator.name = string_part_for_loop[name_index_from_start..name_index_from_end].to_string();
                        arxiv_post.creators.push(creator);
                        string_part_for_loop = string_part_for_loop[name_index_from_end + end_index_creator_name_string.len()..].to_string();
                        creators_count += 1;
                    } else {
                        let mut creator = Creator::new();
                        creator.link = string_part_for_loop[link_index_from_start + start_index_creator_link_string.len()..link_index_from_end].to_string();
                        creators_count += 1;
                        println!(
                    "error to find name_index_from_end, but name_index_from_start(what equals to link_index_from_end + its len) was found, Creator pushed with default creator.name, creators_count={}",
                    creators_count
                );
                    }
                } else {
                    creators_count += 1;
                    println!(
                    "error to find link_index_from_end, but link_index_from_start was found, creators_count={}",
                    creators_count      
                );
                break
                }
            }
        }
        vec_of_arxiv_posts.push(arxiv_post);
        posts_count += 4;
    }
    
    // if vec_of_arxiv_posts.len() == 0{
    //     println!("0 elements in {}",key); 
    // }
    // else if vec_of_arxiv_posts[0].creators.len() == 0{         
    //         println!("0 creators in {}", key);
    // }
    // else {
    //     println!("{} elements in key {}", posts_count,  key);
    // }
    vec_of_arxiv_posts
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