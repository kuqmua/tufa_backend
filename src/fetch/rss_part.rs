extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::check_net::check_link::check_link;
use crate::fetch::rss_fetch_and_parse_xml::rss_fetch_and_parse_xml;
use crate::fetch::rss_handle_unfiltered_posts::handle_unfiltered_posts;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::overriding::prints::print_error_red;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::fetch::rss_check_available_providers::rss_check_available_providers;
use crate::fetch::rss_divide_to_equal_for_each_provider::rss_divide_to_equal_for_each_provider;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::get_group_names::get_biorxiv_links::get_biorxiv_links;
use crate::get_group_names::get_medrxiv_links::get_medrxiv_links;
use crate::get_group_names::get_twitter_providers_names::get_twitter_providers_names;
use crate::get_group_names::get_twitter_subs::get_twitter_subs;

use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rss_structures::RssPostStruct;

pub fn rss_part(
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
            let twitter_available_providers_links: Vec<&str> = rss_check_available_providers(
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
        let twitter_available_providers_links: Vec<&str>;
        match provider_kind {
            ProviderKind::Arxiv => {
                twitter_available_providers_links = Vec::new();
            }
            ProviderKind::Biorxiv => {
                twitter_available_providers_links = Vec::new();
            }
            ProviderKind::Medrxiv => {
                twitter_available_providers_links = Vec::new();
            }
            ProviderKind::Twitter => {
                let twitter_providers_names: Vec<&str> = get_twitter_providers_names();
                // let twitter_available_providers_links: Vec<String> =
                twitter_available_providers_links = rss_check_available_providers(
                    enable_prints,
                    enable_error_prints,
                    enable_time_measurement,
                    twitter_providers_names,
                );
            }
        }
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
                links_temp_naming = get_twitter_subs(twitter_available_providers_links.clone());
            }
        }
        if !links_temp_naming.is_empty() {
            let links_len = links_temp_naming.len();
            let unfiltered_posts_hashmap_after_fetch_and_parse: Vec<(
                String,
                (
                    RssPostStruct,
                    String,
                    UnhandledFetchStatusInfo,
                    HandledFetchStatusInfo,
                    AreThereItems,
                ),
            )>;
            match provider_kind {
                ProviderKind::Arxiv => {
                    unfiltered_posts_hashmap_after_fetch_and_parse = rss_fetch_and_parse_xml(
                        enable_prints,
                        enable_error_prints,
                        enable_time_measurement,
                        links_temp_naming,
                        provider_kind,
                    );
                }
                ProviderKind::Biorxiv => {
                    unfiltered_posts_hashmap_after_fetch_and_parse = rss_fetch_and_parse_xml(
                        enable_prints,
                        enable_error_prints,
                        enable_time_measurement,
                        links_temp_naming,
                        provider_kind,
                    );
                }
                ProviderKind::Medrxiv => {
                    unfiltered_posts_hashmap_after_fetch_and_parse = rss_fetch_and_parse_xml(
                        enable_prints,
                        enable_error_prints,
                        enable_time_measurement,
                        links_temp_naming,
                        provider_kind,
                    );
                }
                ProviderKind::Twitter => {
                    let vec_of_hashmap_parts = rss_divide_to_equal_for_each_provider(
                        twitter_available_providers_links,
                        links_temp_naming,
                        links_len,
                    );
                    let not_ready_processed_posts =
                        Arc::new(Mutex::new(Vec::with_capacity(links_len)));
                    let mut threads_vector = Vec::with_capacity(vec_of_hashmap_parts.len());
                    for element in &mut vec_of_hashmap_parts.into_iter() {
                        let not_ready_processed_posts_handle =
                            Arc::clone(&not_ready_processed_posts);
                        let thread = thread::spawn(move || {
                            let unfiltered_posts_hashmap_after_fetch_and_parse =
                                rss_fetch_and_parse_xml(
                                    enable_prints,
                                    enable_error_prints,
                                    enable_time_measurement,
                                    element.clone(),
                                    &provider_kind,
                                );
                            let mut locked_not_ready_processed_posts =
                                not_ready_processed_posts_handle.lock().unwrap();
                            for (key, value) in unfiltered_posts_hashmap_after_fetch_and_parse {
                                locked_not_ready_processed_posts.push((key, value));
                            }
                        });
                        threads_vector.push(thread);
                    }
                    for thread in threads_vector {
                        thread.join().unwrap();
                    }
                    let f = &*not_ready_processed_posts.lock().unwrap().to_vec();
                    unfiltered_posts_hashmap_after_fetch_and_parse = f.to_vec();
                }
            }

            handle_unfiltered_posts(
                unfiltered_posts_hashmap_after_fetch_and_parse,
                provider_kind,
                enable_prints,
                enable_warning_prints,
                enable_error_prints,
                enable_cleaning_logs_directory,
                enable_time_measurement,
            )
        } else {
            if enable_error_prints {
                print_error_red(
                    file!().to_string(),
                    line!().to_string(),
                    "twitter_links.is_empty".to_string(),
                );
            }
            false
        }
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

// if enable_prints {
//                 println!(
//                     "thread::spawn for each provider must be done {:#?} times...",
//                     vec_of_hashmap_parts.len()
//                 );
//             }

// if enable_prints {
//                 println!(
//                     "twitter providers available {:#?} of {:#?}",
//                     twitter_available_providers_links_len, twitter_providers_names_length_for_debug
//                 );
//                 println!(
//                     "links({}) for each provider ({}) links_for_each_provider {} links_to_remaind {}",
//                     links.len(),
//                     twitter_available_providers_links_len,
//                     links.len() / twitter_available_providers_links_len,
//                     links.len() % twitter_available_providers_links_len,
//                 );
//             }
