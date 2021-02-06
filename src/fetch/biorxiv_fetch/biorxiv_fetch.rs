extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use super::biorxiv_fetch_and_parse_xml::biorxiv_fetch_and_parse_xml;
use crate::check_net::check_link::check_link;
use crate::config::BIORXIV_URL;
use crate::config::ENABLE_PRINTS_BIORXIV;

pub fn biorxiv_part() -> bool {
    if check_link(BIORXIV_URL).0 {
        if ENABLE_PRINTS_BIORXIV {
            println!("i can reach {}", BIORXIV_URL);
        };
        let fff = biorxiv_fetch_and_parse_xml();
        if ENABLE_PRINTS_BIORXIV {
            println!("{:#?} elements in Biorxiv HashMap", fff.len())
        };
        true
    } else {
        if ENABLE_PRINTS_BIORXIV {
            println!("i cannot reach {}", BIORXIV_URL);
        };
        return false;
    }
}
