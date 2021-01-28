// use crossbeam::scope;

// use futures::future;
// use reqwest::{Client, Response};
// use serde_xml_rs::from_str;
// use std::collections::hash_map::RandomState;
// use std::collections::HashMap;
// use std::str;
// use std::sync::Arc;
// use std::sync::Mutex;
// use std::thread;
// use std::time::Instant;
// use tokio;
// // use xml::Element;
// // use xml::Xml;

// use super::arxiv_structures::ArxivPost;
// use super::arxiv_structures::ArxivPostStruct;
// use super::arxiv_structures::Creator;
// use super::arxiv_structures::XmlArxivParserStruct;
// use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
// use crate::config::ENABLE_PRINTS_ARXIV;
// use crate::get_group_names::get_arxiv_links::get_arxiv_links;
// use crate::override_prints::override_prints::print_error_red;

// #[tokio::main]
// pub async fn do_something() -> Arc<Mutex<HashMap<String, ArxivPostStruct, RandomState>>> {
//     let time = Instant::now();
//     let client = Client::new();
//     let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links();
//     let vec_of_links: Vec<&str> = arxiv_links_in_hash_map.values().cloned().collect();
//     let vec_of_keys: Vec<&str> = arxiv_links_in_hash_map.keys().cloned().collect();
//     let vec_links_len = vec_of_links.len();
//     let arcmap: HashMap<String, ArxivPostStruct> = HashMap::with_capacity(vec_links_len);
//     println!("1");
//     let arcmap = Arc::new(Mutex::new(arcmap));
//     tokio::spawn(async move {
//         let arcmapt = arcmap.clone();
//         let resp = client.get(vec_of_links[0]).send().await;
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
// use std::collections::HashMap;
// use std::sync::Arc;
// use std::sync::Mutex;
// use tokio;
// #[tokio::main]
// pub async fn do_something() -> HashMap<String, String> {
//     let hsmap: HashMap<String, String> = HashMap::new();
//     let arcmap = Arc::new(Mutex::new(hsmap));
//     let handle = {
//         let arcmap = arcmap.clone();
//         tokio::spawn(async move {
//             arcmap
//                 .lock()
//                 .unwrap()
//                 .insert("something".to_string(), "something".to_string());
//         });
//     };
//     let arcmap: HashMap<String, String> = Arc::try_unwrap(arcmap).unwrap().into_inner().unwrap();
//     arcmap
// }
// use std::sync::{Arc, Mutex};
// use std::thread;

// async fn thread_func(results: Arc<Mutex<Vec<i32>>>, thread_id: i32) {
//     println!("dsg");
//     let mut results = results.lock().unwrap();

//     results[thread_id as usize] = 5;
//     println!("dsgsdsd {:#?}", results);
// }

// pub fn do_something() -> Arc<Mutex<Vec<i32>>> {
//     let results = Arc::new(Mutex::new(vec![0; 5]));

//     let guards: Vec<_> = (0..5)
//         .map(|i| {
//             let results = results.clone();
//             thread::spawn(move || thread_func(results, i))
//         })
//         .collect();

//     for guard in guards {
//         guard.join();
//     }

//     results
// }
// use super::arxiv_structures::ArxivPost;
// use super::arxiv_structures::ArxivPostStruct;
// // use super::arxiv_structures::Creator;
// // use super::arxiv_structures::XmlArxivParserStruct;
// use std::collections::HashMap;
// pub fn do_something() -> Vec<ArxivPostStruct> {
//     let vec_links_len = 32;
//     // Vec::with_capacity(5);
//     let useless_arxiv_page_struct_filler = ArxivPostStruct::new();
//     // let mut arcmap: HashMap<String, ArxivPostStruct> = HashMap::with_capacity(vec_links_len);
//     let mut results = vec![useless_arxiv_page_struct_filler; vec_links_len];
//     let mut o = 0;
//     let client = Client::new();
//     // use std::sync::Arc;
//     // use std::sync::Mutex;

//     crossbeam::scope(|scope| {
//         for i in &mut results {
//             scope.spawn(move |_| {
//                 // *i += 1;
//                 let resp = client.get(vec_of_links[0]).send().await;
//                 println!("ss");
//                 let useless_arxiv_page_struct = ArxivPostStruct::new();
//                 // arcmap
//                 //     .lock()
//                 //     .unwrap()
//                 //     .insert(vec_of_keys[0].to_string(), useless_arxiv_page_struct);
//                 *i = useless_arxiv_page_struct;
//             });
//             o += 1;
//         }
//     });

//     results
// }

// use futures::future;

// use serde_xml_rs::from_str;
// use std::collections::hash_map::RandomState;
use std::collections::HashMap;
// use std::str;
use std::sync::Arc;
use std::sync::Mutex;
// use std::thread;
// use std::time::Instant;

// use tokio::sync::mpsc::channel;

// use tokio::sync::mpsc;
use std::thread;
// use tokio::runtime::Runtime;
// use tokio::runtime::Runtime;
// use tokio::sync::mpsc;
// #[tokio::main]
use futures::future::Future;

// use crossbeam::scope;
// use reqwest::blocking;
use reqwest::{self, Response};
use std::error::Error;
use tokio;
use tokio::task;
pub fn do_something() -> HashMap<String, i32> {
    //
    //Vec<i32>
    let mut arxiv_sections_links: HashMap<String, i32> = [
        ("http://export.arxiv.org/rss/astro-ph.CO".to_string(), 0),
        ("http://export.arxiv.org/rss/astro-ph.EP".to_string(), 0),
        ("http://export.arxiv.org/rss/astro-ph.GA".to_string(), 0),
    ]
    .iter()
    .cloned()
    .collect();
    let mut results = vec![0; 2];
    // let results = vec![
    //     "http://export.arxiv.org/rss/astro-ph.CO",
    //     "http://export.arxiv.org/rss/astro-ph.EP",
    // ];
    let mut h = 0;
    let gg = crossbeam::scope(|scope| {
        for (key, value) in &mut arxiv_sections_links {
            scope.spawn(move |_| {
                // let (tx, mut rx) = tokio::sync::mpsc::channel(1);
                // task::spawn(async move {
                //     tx.send(getreesp().await);
                // });
                // let res = rx.blocking_recv();

                // match resp {
                //     Ok(file) => file,
                //     Err(e) => return Err(e),
                // };
                // .json::<HashMap<String, String>>();
                let ggg = marr(key);
                println!("ggg {:#?}", ggg);
                // let f = getreesp();
                // match f {
                //     Some(s) => {
                //         println!("1");
                //         return true;
                //     }
                //     N => {
                //         println!("2");
                //         return false;
                //     }
                // }
                // println!("{:#?}", f);
                *value += 1;
            });
            h += 1;
        }
    });
    println!("arxiv_sections_links {:#?}", arxiv_sections_links);
    arxiv_sections_links
    // results
}
// fn block_on_nnaa<F: Future>(f: F) -> F::Output {
//     let (tx, rx) = tokio::sync::mpsc::channel(1);
//     tokio::task::spawn(async move {
//         tx.send(f.await);
//     });
//     rx.blocking_recv().expect("future panicked")
// }
// async fn getreesp() -> bool {
//     let body = reqwest::blocking::get(); //.text()?
//     match body {
//         Ok(s) => {
//             println!("{:#?}", s.status() == reqwest::StatusCode::OK);
//             return true;
//         }
//         Err(e) => {
//             println!("{:#?}", e);
//             return false;
//         }
//     }
//     // Ok(())
//     //.text()?
//     // match resp {
//     //     Some(suc) => println!("fi"),
//     //     None => println!("f"),
//     // }
// }
fn marr(link: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut res = reqwest::blocking::get(link)?;

    println!("Status: {}", res.status());
    // println!("Body:\n{:?}", );
    let b: String;
    let s = res.text();
    match s {
        Ok(norm) => {
            println!("{}", norm);
            b = norm;
        }
        Err(e) => {
            println!("err{}", e);
            b = "fetcherr".to_string();
        }
    }

    // copy the response body directly to stdout
    // res.copy_to(&mut std::io::stdout())?;

    println!("\n\nDone.");
    Ok(b)
}

// pub async fn do_something(
// ) -> Arc<std::sync::Mutex<HashMap<std::string::String, std::string::String>>> {
//     // let mut results: Vec<i32> = vec![0, 0, 0, 0];
//     let hsmap: HashMap<String, String> = HashMap::new();
//     let arcmap = Arc::new(Mutex::new(hsmap));
//     let mut threads_vec = vec![];
//     // let arcmap = Arc::new(Mutex::new(results));
//     threads_vec.push(thread::spawn(move || {
//         let (tx, mut rx) = tokio::sync::mpsc::channel(1);
//         println!("2");
//         // *i += 1;
//         // arcmap.lock().unwrap();

//         // arcmap[0] += 1;
//         //                                     .insert(vec_of_keys[0].to_string(), arxiv_page_struct);
//         &arcmap
//             .lock()
//             .unwrap()
//             .insert("something".to_string(), "something".to_string());
//         println!("1");
//         task::spawn(async move {
//             tx.send(getreesp().await);
//         });
//         println!("3");
//         let res = rx.blocking_recv();
//     }));

//     arcmap
// }
