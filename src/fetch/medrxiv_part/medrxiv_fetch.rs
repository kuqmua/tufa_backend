extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::check_net::check_link::check_link;
use crate::config::ENABLE_ERROR_PRINTS_MEDRXIV;
use crate::config::ENABLE_PRINTS_MEDRXIV;
use crate::config::MEDRXIV_URL;
use crate::fetch::rxiv_fetch_and_parse_xml::rxiv_fetch_and_parse_xml;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::get_group_names::get_medrxiv_links::get_medrxiv_links;

pub fn medrxiv_part() -> bool {
    if check_link(MEDRXIV_URL).0 {
        if ENABLE_PRINTS_MEDRXIV {
            println!("i can reach {}", MEDRXIV_URL);
        };
        println!("medrxiv;");
        let fff = rxiv_fetch_and_parse_xml(
            ENABLE_PRINTS_MEDRXIV,
            ENABLE_ERROR_PRINTS_MEDRXIV,
            get_medrxiv_links(),
            RxivKind::Medrxiv,
        );
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
