extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::check_net::check_link::check_link;
use crate::config::WARNING_LOGS_DIRECTORY_NAME;
use crate::fetch::provider_kind_enum::ProviderKind;
use crate::fetch::twitter_async_write_fetch_error_logs_into_files_wrapper::twitter_async_write_fetch_error_logs_into_files_wrapper;
use crate::fetch::twitter_fetch_and_parse_xml::twitter_fetch_and_parse_xml;
use crate::fetch::twitter_filter_fetched_and_parsed_posts::twitter_filter_fetched_and_parsed_posts;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_partial_success_cyan;
use crate::overriding::prints::print_success_green;
use crate::overriding::prints::print_warning_orange;
use futures::executor::block_on;
use std::collections::HashMap;
use std::fs;
use std::mem;
use std::path::Path;
use std::thread;

use crate::fetch::twitter::twitter_check_available_providers::twitter_check_available_providers;
use crate::get_group_names::get_twitter_providers_names::get_twitter_providers_names;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::get_group_names::get_biorxiv_links::get_biorxiv_links;
use crate::get_group_names::get_medrxiv_links::get_medrxiv_links;

pub fn rxiv_part(
    enable_cleaning_logs_directory: bool,
    enable_prints: bool,
    enable_warning_prints: bool,
    enable_error_prints: bool,
    enable_time_measurement: bool,
    provider_link: &str,
    provider_kind: &'static ProviderKind,
) -> bool {
    let mut availability_checker_flag: bool = false;
    match provider_kind {
        ProviderKind::Arxiv => {
            if check_link(provider_link).0 {
                availability_checker_flag = true;
            }
        }
        ProviderKind::Biorxiv => {
            if check_link(provider_link).0 {
                availability_checker_flag = true;
            }
        }
        ProviderKind::Medrxiv => {
            if check_link(provider_link).0 {
                availability_checker_flag = true;
            }
        }
        ProviderKind::Twitter => {
            let twitter_providers_names: Vec<&str> = get_twitter_providers_names();
            let twitter_available_providers_links: Vec<&str> = twitter_check_available_providers(
                enable_prints,
                enable_error_prints,
                enable_time_measurement,
                twitter_providers_names,
            );
            if !twitter_available_providers_links.is_empty() {
                availability_checker_flag = true;
            }
        }
    }
    if availability_checker_flag {
        if enable_prints {
            println!("i can reach {}", provider_link)
        };
        let links_temp_naming: HashMap<&str, String>;
        match provider_kind {
            ProviderKind::Arxiv => {
                links_temp_naming = get_arxiv_links();
            }
            ProviderKind::Biorxiv => {
                links_temp_naming = get_biorxiv_links();
            }
            ProviderKind::Medrxiv => {
                links_temp_naming = get_medrxiv_links();
            }
            ProviderKind::Twitter => {
                links_temp_naming = HashMap::new();
                // panic!("twitter not handled yet!")
            }
        }
        let unfiltered_posts_hashmap_after_fetch_and_parse = twitter_fetch_and_parse_xml(
            enable_prints,
            enable_error_prints,
            enable_time_measurement,
            links_temp_naming,
            provider_kind,
        );
        let unfiltered_posts_hashmap_after_fetch_and_parse_len_counter =
            unfiltered_posts_hashmap_after_fetch_and_parse.len();
        let (
            unhandled_success_handled_success_are_there_items_yep_posts,
            some_error_posts,
        ) = twitter_filter_fetched_and_parsed_posts(
            unfiltered_posts_hashmap_after_fetch_and_parse,
            &provider_kind,
        );

        if unhandled_success_handled_success_are_there_items_yep_posts.is_empty() {
            let error_message = format!(
                "unhandled_success_handled_success_are_there_items_yep_posts is EMPTY!!! {}",
                provider_link
            );
            if enable_warning_prints {
                print_warning_orange(file!().to_string(), line!().to_string(), error_message);
            }
            // false
        } else if unhandled_success_handled_success_are_there_items_yep_posts.len()
            != unfiltered_posts_hashmap_after_fetch_and_parse_len_counter
        {
            let wrong_cases_thread = thread::spawn(move || {
                if enable_prints {
                    let message = format!(
                        "(partially)succesfully_fetched_and_parsed_posts {} out of {} for {:#?}, allocated: {} byte/bytes",
                        unhandled_success_handled_success_are_there_items_yep_posts.len(),
                        unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
                        provider_kind,
                        mem::size_of_val(&unhandled_success_handled_success_are_there_items_yep_posts)
                    );
                    print_partial_success_cyan(file!().to_string(), line!().to_string(), message);
                }
                if enable_cleaning_logs_directory {
                    let path = format!("logs/{}/{:?}", WARNING_LOGS_DIRECTORY_NAME, provider_kind);
                    if Path::new(&path).is_dir() {
                        let result_of_recursively_removing_warning_logs_directory =
                            fs::remove_dir_all(&path);
                        match result_of_recursively_removing_warning_logs_directory {
                            Ok(_) => {
                                if enable_prints {
                                    println!("папка {} удалена", &path);
                                }
                            }
                            Err(e) => {
                                if enable_error_prints {
                                    let message = format!(
                                        "проблема с удалением папки {} {}",
                                        &path,
                                        e.to_string()
                                    );
                                    print_error_red(
                                        file!().to_string(),
                                        line!().to_string(),
                                        message,
                                    )
                                }
                            }
                        }
                    }
                }
                twitter_async_write_fetch_error_logs_into_files_wrapper(
                    provider_kind,
                    enable_prints,
                    // enable_warning_prints,
                    enable_error_prints,
                    enable_time_measurement,
                    some_error_posts,
                );
            });
            wrong_cases_thread.join().unwrap();
            // true
        } else {
            if enable_prints {
                let message = format!(
                    "succesfully_fetched_and_parsed_posts {} out of {} for {:#?}, allocated: {} byte/bytes",
                    unhandled_success_handled_success_are_there_items_yep_posts.len(),
                    unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
                    provider_kind,
                    mem::size_of_val(&unhandled_success_handled_success_are_there_items_yep_posts)
                );
                print_success_green(file!().to_string(), line!().to_string(), message);
            }
            // true
        }
        true
    } else {
        if enable_error_prints {
            match provider_kind {
                ProviderKind::Arxiv => {
                    let error_message =
                        format!("i cannot reach {} for {:#?}", provider_link, provider_kind);
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
                ProviderKind::Biorxiv => {
                    let error_message =
                        format!("i cannot reach {} for {:#?}", provider_link, provider_kind);
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
                ProviderKind::Medrxiv => {
                    let error_message =
                        format!("i cannot reach {} for {:#?}", provider_link, provider_kind);
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
                ProviderKind::Twitter => {
                    let error_message = format!(
                        "i cannot reach any of provider links for {:#?}",
                        provider_kind
                    );
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
            }
        };
        false
    }
}
