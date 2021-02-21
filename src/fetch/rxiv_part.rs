extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::check_net::check_link::check_link;
use crate::fetch::rxiv_fetch_and_parse_xml::rxiv_fetch_and_parse_xml;
use crate::fetch::rxiv_kind_enum::RxivKind;
use std::collections::HashMap;

pub fn rxiv_part(
    links: HashMap<&'static str, &'static str>,
    enable_prints: bool,
    enable_error_prints: bool,
    rxiv_url: &str,
    rxiv_kind: RxivKind,
) -> bool {
    if check_link(rxiv_url).0 {
        if enable_prints {
            println!("i can reach {}", rxiv_url);
        };
        let rxiv_kind_clone = rxiv_kind.clone();
        let fff = rxiv_fetch_and_parse_xml(enable_prints, enable_error_prints, links, rxiv_kind);
        if enable_prints {
            println!(
                "{:#?} elements in {:#?} HashMap",
                fff.len(),
                rxiv_kind_clone
            );
        };
        true
    } else {
        if enable_prints {
            println!("i cannot reach {}", rxiv_url);
        };
        false
    }
}
