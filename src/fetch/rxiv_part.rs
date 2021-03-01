extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::check_net::check_link::check_link;
use crate::fetch::handle_error_status_code::handle_error_status_code;
use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rxiv_fetch_and_parse_xml::rxiv_fetch_and_parse_xml;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::fetch::rxiv_structures::RxivPostStruct;
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
        let zzz = rxiv_kind.clone(); //only for debug
        let rxiv_kind_clone = rxiv_kind.clone();

        let fff = rxiv_fetch_and_parse_xml(enable_prints, enable_error_prints, links, rxiv_kind);
        let fff_len_counter = fff.len();
        if enable_prints {
            println!(
                "{:#?} elements in {:#?} HashMap",
                fff.len(),
                rxiv_kind_clone
            );
        };
        let mut succesfully_fetched_and_parsed_posts: HashMap<String, (RxivPostStruct, RxivKind)> =
            HashMap::new();
        let mut initialized_items_posts: HashMap<String, (String, RxivKind)> = HashMap::new();
        let mut not_items_but_tag_posts: HashMap<String, (String, RxivKind)> = HashMap::new(); //"</item>" tag
        for (key, value) in fff {
            match value.2 {
                UnhandledFetchStatusInfo::Success => match value.3 {
                    HandledFetchStatusInfo::Success => match value.4 {
                        AreThereItems::Yep => {
                            succesfully_fetched_and_parsed_posts.insert(key, (value.0, value.5));
                        }
                        AreThereItems::Initialized => {
                            initialized_items_posts.insert(key, (value.1, value.5));
                        }
                        AreThereItems::NopeButThereIsTag(String) => {
                            //"</item>" tag
                            not_items_but_tag_posts.insert(key, (value.1, value.5));
                        }
                        AreThereItems::ConversionFromStrError(something1, something2) => {}
                        AreThereItems::NopeNoTag(String) => {}
                    },
                    HandledFetchStatusInfo::Initialized => {}
                    HandledFetchStatusInfo::ResToTextError(String) => {}
                    HandledFetchStatusInfo::ResStatusError(status_code) => {
                        let should_refetch_it = handle_error_status_code(status_code);
                        //.as_u16()
                    }
                },
                UnhandledFetchStatusInfo::Initialized => {}
                UnhandledFetchStatusInfo::Failure(string_failure) => {}
            }
        }
        println!(
            "succesfully_fetched_and_parsed_posts {} out of {} for {:#?}",
            succesfully_fetched_and_parsed_posts.len(),
            fff_len_counter,
            zzz
        );
        true
    } else {
        if enable_prints {
            println!("i cannot reach {}", rxiv_url);
        };
        false
    }
}
