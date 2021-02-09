extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use super::medrxiv_fetch_and_parse_xml::medrxiv_fetch_and_parse_xml;
use crate::check_net::check_link::check_link;
use crate::config::ENABLE_PRINTS_MEDRXIV;
use crate::config::MEDRXIV_URL;

pub fn medrxiv_part() -> bool {
    if check_link(MEDRXIV_URL).0 {
        if ENABLE_PRINTS_MEDRXIV {
            println!("i can reach {}", MEDRXIV_URL);
        };
        let fff = medrxiv_fetch_and_parse_xml();
        if ENABLE_PRINTS_MEDRXIV {
            println!("{:#?} elements in Medrxiv HashMap", fff.len())
        };
        true
    } else {
        if ENABLE_PRINTS_MEDRXIV {
            println!("i cannot reach {}", MEDRXIV_URL);
        };
        false
    }
}
