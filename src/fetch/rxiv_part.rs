extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::check_net::check_link::check_link;
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
        let rxiv_kind_clone = rxiv_kind.clone();
        let fff = rxiv_fetch_and_parse_xml(enable_prints, enable_error_prints, links, rxiv_kind);
        if enable_prints {
            println!(
                "{:#?} elements in {:#?} HashMap",
                fff.len(),
                rxiv_kind_clone
            );
        };
        let succesfully_fetched_and_parsed_posts: HashMap<String, (RxivPostStruct, RxivKind)> =
            HashMap::new();
        for (_, value) in fff {
            // println!(
            //     "unhandled {:#?} - handled {:#?} - arethereitems {:#?}",
            //     value.2, value.3, value.4
            // );
            match value.2 {
                UnhandledFetchStatusInfo::Success => match value.3 {
                    HandledFetchStatusInfo::Success => match value.4 {
                        AreThereItems::Yep => {}
                        AreThereItems::Initialized => {}
                        AreThereItems::NopeButThereIsTag(String) => {}
                        AreThereItems::ConversionFromStrError(something1, something2) => {}
                        AreThereItems::NopeNoTag(String) => {}
                    },
                    HandledFetchStatusInfo::Initialized => {}
                    HandledFetchStatusInfo::ResToTextError(String) => {}
                    HandledFetchStatusInfo::ResStatusError(status_code) => {}
                },
                UnhandledFetchStatusInfo::Initialized => {}
                UnhandledFetchStatusInfo::Failure(string_failure) => {}
            }
        }
        true
    } else {
        if enable_prints {
            println!("i cannot reach {}", rxiv_url);
        };
        false
    }
}
