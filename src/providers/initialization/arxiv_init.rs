//СТАРАЯ ВЕРСИЯ АРХИВА
// use std::collections::HashMap;
// extern crate reqwest;
// extern crate xml;
// use std::time::Instant;
// use std::fmt::Display;
// use quick_xml::events::Event;
// use quick_xml::Reader;
// /*
// #[path = "../parsing/arxiv/parse_arxiv/get_arxiv_posts.rs"]
// mod get_arxiv_posts;
// use get_arxiv_posts::get_arxiv_posts;

// #[path = "../parsing/arxiv/arxiv_xml_structs/arxiv_post.rs"]
// mod arxiv_post;
// use arxiv_post::ArxivPost;
// */
// #[path = "./get_group_names/get_arxiv_links_in_hash_map.rs"]
// mod get_arxiv_links_in_hash_map;
// use get_arxiv_links_in_hash_map::get_arxiv_links_in_hash_map;

// pub fn arxiv_init() -> std::collections::HashMap<&'static str, Vec<ArxivPost>>
// {
//     let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links_in_hash_map();
//     let mut arxiv_pages_posts_hashmap = HashMap::new();
//     println!("{:#?}", arxiv_links_in_hash_map.len());

//     let mut increment_for_errors = 0;
//     for (key, value) in arxiv_links_in_hash_map {
//         println!("starting to fetch data with index {}", increment_for_errors);
//         println!("fetching url now ... {}", value);
//         let time = Instant::now();
//         let result_of_arxiv_posts = get_arxiv_link_posts(value); //
//         match result_of_arxiv_posts {
//             Ok(vec_of_arxiv_page_posts) => {
//                 arxiv_pages_posts_hashmap.insert(key, vec_of_arxiv_page_posts);
//                 println!(
//                     "iteration with index {} just fetched ok data",
//                     increment_for_errors
//                 );
//                 increment_for_errors += 1;
//             }
//             Err(e) => {
//                 println!(
//                     "error = {}, iteration with index  = {}",
//                     e, increment_for_errors
//                 );
//                 increment_for_errors += 1;
//             }
//         }
//         println!(
//             "iteration working/fetching(in seconds) = {} ",
//             time.elapsed().as_secs()
//         );
//         //map.remove(key);
//     }
//     println!("{:#?}", arxiv_pages_posts_hashmap);
//     arxiv_pages_posts_hashmap
// }

// pub fn get_arxiv_link_posts(link: &str) -> Result<Vec<ArxivPost>, Box<dyn std::error::Error>> {
//     let vec_of_string_fetching_result = get_vec_of_strings_from_arxiv_rss_link(link);
//     match vec_of_string_fetching_result{
//         Ok(vec_of_arxiv_rss_strings_what_contains_xmls) => {
//                let vec_of_arxiv_rss_strings_what_contains_xmls_removed = remove_meta_elements_from_vec_of_arxiv_rss(vec_of_arxiv_rss_strings_what_contains_xmls);
//     let vec_of_arxiv_posts: Vec<ArxivPost> = parse_arxiv_string_xml(vec_of_arxiv_rss_strings_what_contains_xmls_removed);
//     if vec_of_arxiv_posts.len() == 0{
//         println!("vec_of_arxiv_posts.len() == 0");
//     }
//     else{

//         if vec_of_arxiv_posts[0].creators.len() == 0{
//             println!("vec_of_arxiv_posts[0].creators.len() == 0");
//         }
//         else{
//             println!("vec_of_arxiv_posts[0].creators[0].name {:#?}", vec_of_arxiv_posts[0].creators[0].name);
//         }
//     }
//     Ok(vec_of_arxiv_posts)
//         }
//         Err(e) => {
//             println!("error with get_vec_of_strings_from_arxiv_rss_link {}", e);
//             Err(e)
//         }
//     }

// }

// #[tokio::main]
//  async fn get_vec_of_strings_from_arxiv_rss_link(link: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//     let resp = reqwest::get(link)
//         .await?
//         .text()
//         .await?;
//     let xml: String = resp;
//     let mut reader = Reader::from_str(&xml);
//     reader.trim_text(true);
//     let mut vec_of_arxiv_rss_strings_what_contains_xmls = Vec::new();
//     let mut buf = Vec::new();
//     loop {
//         match reader.read_event(&mut buf) {
//             Ok(Event::Start(ref e)) => match e.name() {
//                 b"tag1" => println!(
//                     "attributes values: {:?}",
//                     e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
//                 ),
//                 _ => (),
//             },
//             Ok(Event::Text(e)) => vec_of_arxiv_rss_strings_what_contains_xmls.push(e.unescape_and_decode(&reader).unwrap()),
//             Ok(Event::Eof) => break,
//             Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
//             _ => (),
//         }
//         buf.clear();
//     }
//     Ok(vec_of_arxiv_rss_strings_what_contains_xmls)
// }
// fn remove_meta_elements_from_vec_of_arxiv_rss(mut vec_of_arxiv_rss: Vec<String>)->Vec<String>{
//     let mut remove_element = 0;
//     while remove_element < 13 {
//         vec_of_arxiv_rss.remove(0);
//         remove_element += 1;
//     }
//     vec_of_arxiv_rss
// }

// pub fn parse_arxiv_string_xml(vec_of_arxiv_xml_string: Vec<String>)->Vec<ArxivPost>{
//     let mut vec_of_arxiv_posts: Vec<ArxivPost> = Vec::new();
//     let mut write_count = 0;
//     let end_index_title_string = ". (arXiv";
//     let start_index_description_string = "<p>";
//     let end_index_description_string = "\n</p>";
//     let start_index_creator_link_string = "<a href=\"";
//     let end_index_creator_link_string = "\">";//there is no start_index_creator_name_string because its end_index_creator_link_string + its len()
//     //add time from arxiv
//     let end_index_creator_name_string = "</a>";
//     while write_count < vec_of_arxiv_xml_string.len() {
//         let mut arxiv_post: ArxivPost = ArxivPost::new();
//         let start_title = vec_of_arxiv_xml_string[write_count].clone();
//         if let Some(index) = start_title.find(end_index_title_string) {
//             arxiv_post.title = start_title[..index].to_string();
//         } else {
//             arxiv_post.title = start_title;
//             println!("whole title written and start.find(. (arXiv) works uncorrectly");
//         }
//         arxiv_post.link = vec_of_arxiv_xml_string[write_count + 1].clone();
//         let description_start = vec_of_arxiv_xml_string[write_count + 2].clone();
//         if let Some(index_from_start) = description_start.find(start_index_description_string) {
//             if let Some(index_from_end) = description_start.find(end_index_description_string) {
//                 arxiv_post.description =
//                     description_start[index_from_start + start_index_description_string.len()..index_from_end].to_string();
//             } else {
//                 arxiv_post.description =
//                     description_start[index_from_start + start_index_description_string.len()..].to_string();
//                 println!(
//                     "error to find index_from_end, but index_from_start was found, write_count={}",
//                     write_count
//                 );
//             }
//         } else {
//             if let Some(index_from_end) = description_start.find(end_index_description_string) {
//                 arxiv_post.description = description_start[..index_from_end].to_string();
//                 println!(
//                     "error to find index_from_start, but index_from_end was found, write_count={}",
//                     write_count
//                 );
//             } else {
//                 arxiv_post.description = description_start;
//                 println!("whole title written and description_start.find() worked uncorrectly, write_count={}", write_count);
//             }
//         }
//         let creators_string = vec_of_arxiv_xml_string[write_count + 3].clone();
//         let mut string_part_for_loop = creators_string;
//         let mut creators_count = 0;
//         while let Some(link_index_from_start) = string_part_for_loop.find(start_index_creator_link_string) {
//             if creators_count > 256 {
//                 panic!("why creators_count > 256?"); //переписать когда примешь стандарт по которому ошибки обрабатывать будешь
//             } else {
//                 if let Some(link_index_from_end) = string_part_for_loop.find(end_index_creator_link_string) {
//                     if let Some(name_index_from_end) = string_part_for_loop.find(end_index_creator_name_string) {
//                         let mut creator = Creator::new();
//                         creator.link = string_part_for_loop[link_index_from_start + start_index_creator_link_string.len()..link_index_from_end].to_string();
//                         let name_index_from_start = link_index_from_end + end_index_creator_link_string.len();
//                         creator.name = string_part_for_loop[name_index_from_start..name_index_from_end].to_string();
//                         //println!("creator.name = {}", creator.name);
//                         arxiv_post.creators.push(creator);
//                         string_part_for_loop = string_part_for_loop[name_index_from_end + end_index_creator_name_string.len()..].to_string();
//                         creators_count += 1;
//                     } else {
//                         let mut creator = Creator::new();
//                         creator.link = string_part_for_loop[link_index_from_start + start_index_creator_link_string.len()..link_index_from_end].to_string();
//                         creators_count += 1;
//                         println!(
//                     "error to find name_index_from_end, but name_index_from_start(what equals to link_index_from_end + its len) was found, Creator pushed with default creator.name, creators_count={}",
//                     creators_count
//                 );
//                     }
//                 } else {
//                     creators_count += 1;
//                     println!(
//                     "error to find link_index_from_end, but link_index_from_start was found, creators_count={}",
//                     creators_count
//                 );
//                 break
//                 }
//             }
//         }
//         vec_of_arxiv_posts.push(arxiv_post);
//         write_count += 4;
//     }
//     println!("write_count (posts count) {}", write_count);
//     if vec_of_arxiv_posts.len() == 0{
//         println!("vec_of_arxiv_posts.len() == 0");
//     }
//     else{
//         if vec_of_arxiv_posts[0].creators.len() == 0{
//             println!("vec_of_arxiv_posts[0].creators.len() == 0");
//         }
//         else{
//             println!("end of parse_arxiv_string_xml {:#?}", vec_of_arxiv_posts[0].creators[0].name);
//         }
//     }
//     vec_of_arxiv_posts
// }

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// pub struct ArxivPost {
//     pub title: String,
//     pub link: String,
//     pub description: String,
//     pub creators: Vec<Creator>,
// }

// impl Display for ArxivPost {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
//         write!(
//             fmt,
//             "title = {}\nlink = {}\ndescription ={}\ncreators = {:#?}\n",
//             self.title, self.link, self.description, self.creators
//         )
//     }
// }

// impl ArxivPost {
//     pub fn new() -> Self {
//         ArxivPost {
//             title: "".to_string(),
//             link: "".to_string(),
//             description: "".to_string(),
//             creators: Vec::<Creator>::new(),
//         }
//     }
// }

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// pub struct Creator {
//     pub name: String,
//     pub link: String,
// }

// impl Display for Creator {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
//         write!(fmt, "name = {}\nlink = {}\n", self.name, self.link)
//     }
// }

// impl Creator {
//     pub fn new() -> Self {
//         Creator {
//             name: "".to_string(),
//             link: "".to_string(),
//         }
//     }
// }
