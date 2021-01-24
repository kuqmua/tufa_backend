extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use super::medrxiv_fetch_and_parse_xml::medrxiv_fetch_and_parse_xml;
use crate::check_provider::can_i_reach_provider::reach_provider;
use crate::config::ENABLE_PRINTS_MEDRXIV;
use crate::config::MEDRXIV_URL;
use std::collections::HashMap;
use std::str;

use crate::get_group_names::get_medrxiv_links::get_medrxiv_links;

pub fn medrxiv_part() -> bool {
    if reach_provider(MEDRXIV_URL.to_string()) {
        let medrxiv_links_in_hash_map: HashMap<&str, &str> = get_medrxiv_links();
        if ENABLE_PRINTS_MEDRXIV {
            println!(
                "{:#?} elements in Medrxiv HashMap",
                medrxiv_links_in_hash_map.len()
            );
        };
        let vec_of_links: Vec<&str> = medrxiv_links_in_hash_map.values().cloned().collect();
        let vec_of_keys: Vec<&str> = medrxiv_links_in_hash_map.keys().cloned().collect();
        medrxiv_fetch_and_parse_xml(vec_of_links, vec_of_keys); //тут есть возвращаемое значение let vec_of_vec_of_strings =
        return true; //чекнуть действительно ли в векторе есть хоть шот полезное
    } else {
        return false;
    }
}
