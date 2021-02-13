extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::check_net::check_link::check_link;
use crate::config::ARXIV_URL;
use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::fetch::rxiv_fetch_and_parse_xml::rxiv_fetch_and_parse_xml;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;

pub fn arxiv_part() -> bool {
    //shame commit
    if check_link(ARXIV_URL).0 {
        if ENABLE_PRINTS_ARXIV {
            println!("i can reach {}", ARXIV_URL);
        };
        let fff = rxiv_fetch_and_parse_xml(
            ENABLE_PRINTS_ARXIV,
            ENABLE_ERROR_PRINTS_ARXIV,
            get_arxiv_links(),
            RxivKind::Arxiv,
        );
        if ENABLE_PRINTS_ARXIV {
            println!("{:#?} elements in Arxiv HashMap", fff.len());
        };
        true //чекнуть действительно ли в векторе есть хоть шот полезное
    } else {
        if ENABLE_PRINTS_ARXIV {
            println!("i cannot reach {}", ARXIV_URL);
        };
        false
    }
}
