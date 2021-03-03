extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use reqwest::StatusCode;

use crate::check_net::check_link::check_link;
// use crate::fetch::handle_error_status_code::handle_error_status_code;
use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rxiv_fetch_and_parse_xml::rxiv_fetch_and_parse_xml;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::fetch::rxiv_structures::RxivPostStruct;
// use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_warning_orange;
use crate::overriding::prints::print_warning_yellow;
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
        let mut unhandled_success_handled_success_are_there_items_yep_posts: HashMap<
            String,
            (RxivPostStruct, RxivKind),
        > = HashMap::new();
        let mut unhandled_success_handled_success_are_there_items_initialized_posts: HashMap<
            String,
            (String, RxivKind),
        > = HashMap::new();
        let mut unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts: HashMap<String, (String, String, RxivKind)> =
            HashMap::new(); //"</item>" tag
        let mut unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts: HashMap<
            String,
            (String, String, String, RxivKind),
        > = HashMap::new();
        let mut unhandled_success_handled_success_are_there_items_nope_no_tag_posts: HashMap<
            String,
            (String, String, RxivKind),
        > = HashMap::new();
        /////
        let mut unhandled_success_handled_initialized_posts: HashMap<String, (String, RxivKind)> =
            HashMap::new();
        let mut unhandled_success_handled_res_to_text_error_posts: HashMap<
            String,
            (String, String, RxivKind),
        > = HashMap::new();
        let mut unhandled_success_handled_res_status_error_posts: HashMap<
            String,
            (String, StatusCode, RxivKind),
        > = HashMap::new();
        //////
        let mut unhandled_initialized_posts: HashMap<String, (String, RxivKind)> = HashMap::new();
        let mut unhandled_failure_posts: HashMap<String, (String, String, RxivKind)> =
            HashMap::new();
        for (key, value) in fff {
            match value.2 {
                UnhandledFetchStatusInfo::Success => match value.3 {
                    HandledFetchStatusInfo::Success => match value.4 {
                        AreThereItems::Yep => {
                            unhandled_success_handled_success_are_there_items_yep_posts
                                .insert(key, (value.0, value.5));
                        }
                        AreThereItems::Initialized => {
                            unhandled_success_handled_success_are_there_items_initialized_posts
                                .insert(key, (value.1, value.5));
                        }
                        AreThereItems::NopeButThereIsTag(fetch_result_string) => {
                            //"</item>" tag
                            unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts
                                .insert(key, (value.1, fetch_result_string, value.5));
                        }
                        AreThereItems::ConversionFromStrError(fetch_result_string, error) => {
                            unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts
                                .insert(key, (value.1, fetch_result_string, error, value.5));
                        }
                        AreThereItems::NopeNoTag(fetch_result_string) => {
                            unhandled_success_handled_success_are_there_items_nope_no_tag_posts
                                .insert(key, (value.1, fetch_result_string, value.5));
                        }
                    },
                    HandledFetchStatusInfo::Initialized => {
                        unhandled_success_handled_initialized_posts.insert(key, (value.1, value.5));
                    }
                    HandledFetchStatusInfo::ResToTextError(error) => {
                        unhandled_success_handled_res_to_text_error_posts
                            .insert(key, (value.1, error, value.5));
                    }
                    HandledFetchStatusInfo::ResStatusError(status_code) => {
                        // let should_refetch_it = handle_error_status_code(status_code);
                        unhandled_success_handled_res_status_error_posts
                            .insert(key, (value.1, status_code, value.5));
                    }
                },
                UnhandledFetchStatusInfo::Initialized => {
                    unhandled_initialized_posts.insert(key, (value.1, value.5));
                }
                UnhandledFetchStatusInfo::Failure(box_dyn_error) => {
                    unhandled_failure_posts.insert(key, (value.1, box_dyn_error, value.5));
                }
            }
        }
        if unhandled_success_handled_success_are_there_items_yep_posts.is_empty() {
            //do something with it
            print_warning_orange(
                file!().to_string(),
                line!().to_string(),
                "unhandled_success_handled_success_are_there_items_yep_posts is EMPTY!!!"
                    .to_string(),
            );
            false
        } else {
            if !unhandled_success_handled_success_are_there_items_initialized_posts.is_empty() {
                let warning_message =
                    "unhandled_success_handled_success_are_there_items_initialized_posts.len() "
                        .to_string()
                        + &unhandled_success_handled_success_are_there_items_initialized_posts
                            .len()
                            .to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts
                .is_empty()
            {
                let warning_message = "unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts.len() ".to_string()
                    + &unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts.len().to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts
                .is_empty()
            {
                let warning_message = "unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts.len() ".to_string()
                    + &unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts.len().to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_success_handled_success_are_there_items_nope_no_tag_posts.is_empty() {
                let warning_message =
                    "unhandled_success_handled_success_are_there_items_nope_no_tag_posts.len() "
                        .to_string()
                        + &unhandled_success_handled_success_are_there_items_nope_no_tag_posts
                            .len()
                            .to_string();
                for (key, value) in
                    unhandled_success_handled_success_are_there_items_nope_no_tag_posts
                {
                    println!(
                        " HERE key {} \n value 0 {} \n value 1 {}",
                        key, value.0, value.1
                    )
                }
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_success_handled_initialized_posts.is_empty() {
                let warning_message = "unhandled_success_handled_initialized_posts.len() "
                    .to_string()
                    + &unhandled_success_handled_initialized_posts
                        .len()
                        .to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_success_handled_res_to_text_error_posts.is_empty() {
                let warning_message = "unhandled_success_handled_res_to_text_error_posts.len() "
                    .to_string()
                    + &unhandled_success_handled_res_to_text_error_posts
                        .len()
                        .to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_success_handled_res_status_error_posts.is_empty() {
                let warning_message = "unhandled_success_handled_res_status_error_posts.len() "
                    .to_string()
                    + &unhandled_success_handled_res_status_error_posts
                        .len()
                        .to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_initialized_posts.is_empty() {
                let warning_message = "unhandled_initialized_posts.len() ".to_string()
                    + &unhandled_initialized_posts.len().to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_failure_posts.is_empty() {
                let warning_message = "unhandled_failure_posts.len() ".to_string()
                    + &unhandled_failure_posts.len().to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            println!(
                "succesfully_fetched_and_parsed_posts {} out of {} for {:#?}",
                unhandled_success_handled_success_are_there_items_yep_posts.len(),
                fff_len_counter,
                zzz
            );
            true
        }
    } else {
        if enable_prints {
            println!("i cannot reach {}", rxiv_url);
        };
        false
    }
}
