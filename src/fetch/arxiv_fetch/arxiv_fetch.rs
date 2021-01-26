extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use super::arxiv_structures::ArxivPostStruct;
use std::collections::HashMap;
use std::str;

use super::arxiv_fetch_and_parse_xml::do_something; //arxiv_fetch_and_parse_xml
use crate::check_provider::can_i_reach_provider::reach_provider;
use crate::config::ARXIV_URL;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use std::sync::Arc;

pub fn arxiv_part() -> bool {
    // if reach_provider(ARXIV_URL.to_string()) {
    // let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links();
    // if ENABLE_PRINTS_ARXIV {
    //     println!(
    //         "{:#?} elements in Arxiv HashMap",
    //         arxiv_links_in_hash_map.len()
    //     )
    // };
    // let vec_of_links: Vec<&str> = arxiv_links_in_hash_map.values().cloned().collect();
    // let vec_of_keys: Vec<&str> = arxiv_links_in_hash_map.keys().cloned().collect();
    let fff = do_something(); //тут есть возвращаемое значение let vec_of_vec_of_strings =//&vec_of_links, &vec_of_keys
    let jjj: Vec<i32> = Arc::try_unwrap(fff).unwrap().into_inner().unwrap();
    //  HashMap<String, ArxivPostStruct>
    println!("{:#?} elements in Arxiv HashMap", jjj);
    return true; //чекнуть действительно ли в векторе есть хоть шот полезное
                 // } else {
                 // return false;
                 // }
}
