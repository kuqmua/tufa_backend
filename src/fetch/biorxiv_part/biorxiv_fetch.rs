extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::check_net::check_link::check_link;
use crate::config::BIORXIV_URL;
use crate::config::ENABLE_ERROR_PRINTS_BIORXIV;
use crate::config::ENABLE_PRINTS_BIORXIV;
use crate::fetch::rxiv_fetch_and_parse_xml::rxiv_fetch_and_parse_xml;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::get_group_names::get_biorxiv_links::get_biorxiv_links;

pub fn biorxiv_part() -> bool {
    if check_link(BIORXIV_URL).0 {
        if ENABLE_PRINTS_BIORXIV {
            println!("i can reach {}", BIORXIV_URL);
        };
        let fff = rxiv_fetch_and_parse_xml(
            ENABLE_PRINTS_BIORXIV,
            ENABLE_ERROR_PRINTS_BIORXIV,
            get_biorxiv_links(),
            RxivKind::Biorxiv,
        );
        if ENABLE_PRINTS_BIORXIV {
            println!("{:#?} elements in Biorxiv HashMap", fff.len())
        };
        true
    } else {
        if ENABLE_PRINTS_BIORXIV {
            println!("i cannot reach {}", BIORXIV_URL);
        };
        false
    }
}
