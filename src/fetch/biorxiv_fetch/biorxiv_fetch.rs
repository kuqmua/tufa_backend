extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use std::collections::HashMap;
use std::str;

use super::biorxiv_fetch_and_parse_xml::biorxiv_fetch_and_parse_xml;
use crate::check_net::check_link::check_link;
use crate::config::BIORXIV_URL;
use crate::config::ENABLE_PRINTS_BIORXIV;
use crate::get_group_names::get_biorxiv_links::get_biorxiv_links;

pub fn biorxiv_part() -> bool {
    if check_link(BIORXIV_URL).0 {
        let biorxiv_links_in_hash_map: HashMap<&str, &str> = get_biorxiv_links();
        if ENABLE_PRINTS_BIORXIV {
            println!(
                "{:#?} elements in Biorxiv HashMap",
                biorxiv_links_in_hash_map.len()
            )
        };
        let vec_of_links: Vec<&str> = biorxiv_links_in_hash_map.values().cloned().collect();
        let vec_of_keys: Vec<&str> = biorxiv_links_in_hash_map.keys().cloned().collect();
        biorxiv_fetch_and_parse_xml(vec_of_links, vec_of_keys); //тут есть возвращаемое значение let
        return true;
    } else {
        return false;
    }
}
