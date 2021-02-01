extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use super::arxiv_fetch_and_parse_xml::do_something; //arxiv_fetch_and_parse_xml
use crate::check_provider::can_i_reach_provider::reach_provider;
use crate::config::ARXIV_URL;
use crate::config::ENABLE_PRINTS_ARXIV;

pub fn arxiv_part() -> bool {
    if reach_provider(ARXIV_URL.to_string()) {
        let fff = do_something(); //тут есть возвращаемое значение let vec_of_vec_of_strings =//&vec_of_links, &vec_of_keys
        if ENABLE_PRINTS_ARXIV {
            println!("{:#?} elements in Arxiv HashMap", fff.len());
        };
        return true; //чекнуть действительно ли в векторе есть хоть шот полезное
    } else {
        return false;
    }
}
