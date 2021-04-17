extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::config::WARNING_LOGS_DIRECTORY_NAME;
use crate::fetch::provider_kind_enum::ProviderKind;
use crate::fetch::twitter::twitter_async_write_fetch_error_logs_into_files_wrapper::twitter_async_write_fetch_error_logs_into_files_wrapper;
use crate::fetch::twitter::twitter_check_available_providers::twitter_check_available_providers;
use crate::fetch::twitter::twitter_fetch_and_parse_xml::twitter_fetch_and_parse_xml;
use crate::fetch::twitter::twitter_filter_fetched_and_parsed_posts::twitter_filter_fetched_and_parsed_posts;
use crate::get_group_names::get_twitter_providers_names::get_twitter_providers_names;
use crate::get_group_names::get_twitter_subs::get_twitter_subs;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_partial_success_cyan;
use crate::overriding::prints::print_success_green;
use crate::overriding::prints::print_warning_orange;
use futures::executor::block_on;
use std::collections::HashMap;
use std::fs;
use std::mem;
use std::path::Path;

use std::sync::{Arc, Mutex};
use std::thread;

pub fn twitter_part(
    enable_cleaning_logs_directory: bool,
    enable_prints: bool,
    enable_warning_prints: bool,
    enable_error_prints: bool,
    enable_time_measurement: bool,
    provider_link: &str,
    provider_kind: &'static ProviderKind,
) -> bool {
    let twitter_providers_names: Vec<&str> = get_twitter_providers_names();
    let twitter_providers_names_length_for_debug = twitter_providers_names.len();
    // let twitter_available_providers_links: Vec<String> =
    let twitter_available_providers_links: Vec<&str> = twitter_check_available_providers(
        enable_prints,
        enable_error_prints,
        twitter_providers_names,
    );
    if !twitter_available_providers_links.is_empty() {
        let links = get_twitter_subs(twitter_available_providers_links.clone());
        if !links.is_empty() {
            let twitter_available_providers_links_len = twitter_available_providers_links.len();
            if enable_prints {
                println!(
                    "twitter providers available {:#?} of {:#?}",
                    twitter_available_providers_links_len, twitter_providers_names_length_for_debug
                );
                println!(
                    "links({}) for each provider ({}) links_for_each_provider {} links_to_remaind {}",
                    links.len(),
                    twitter_available_providers_links_len,
                    links.len() / twitter_available_providers_links_len,
                    links.len() % twitter_available_providers_links_len,
                );
            }
            let links_len = links.len();
            let links_for_each_provider: usize;
            let is_links_len_more_than_twitter_available_providers_links_len =
                links_len > twitter_available_providers_links_len;
            let vec_of_hashmap_parts_len: usize;
            if is_links_len_more_than_twitter_available_providers_links_len {
                if links_len % twitter_available_providers_links_len == 0 {
                    links_for_each_provider = links_len / twitter_available_providers_links_len;
                } else {
                    //little bit more memory usage than needed but no second allocation!
                    links_for_each_provider =
                        (links_len / twitter_available_providers_links_len) + 1;
                }
                vec_of_hashmap_parts_len = twitter_available_providers_links_len;
            } else {
                links_for_each_provider = links_len;
                vec_of_hashmap_parts_len = links_len;
            }
            let mut vec_of_hashmap_parts: Vec<HashMap<&str, String>> =
                vec![HashMap::with_capacity(links_for_each_provider); vec_of_hashmap_parts_len];
            //HashMap into Vector transformation
            let mut vec_of_links: Vec<(&str, String)> = Vec::with_capacity(links_len);
            for (key, value) in links {
                vec_of_links.push((key, value));
            }
            let mut vec_of_hashmap_parts_element_index_counter = 0;
            let mut even_vec_of_hashmap_parts_element_index_counter = 0;
            let mut even_flag = false;
            if is_links_len_more_than_twitter_available_providers_links_len {
                for element in vec_of_links {
                    if !even_flag {
                        if vec_of_hashmap_parts[vec_of_hashmap_parts_element_index_counter].len()
                            == links_for_each_provider
                        {
                            if (vec_of_hashmap_parts.len() - 1)
                                != vec_of_hashmap_parts_element_index_counter
                            {
                                vec_of_hashmap_parts_element_index_counter += 1;
                                vec_of_hashmap_parts[vec_of_hashmap_parts_element_index_counter]
                                    .insert(element.0, element.1);
                            } else {
                                even_flag = true;
                                vec_of_hashmap_parts
                                    [even_vec_of_hashmap_parts_element_index_counter]
                                    .insert(element.0, element.1);
                                even_vec_of_hashmap_parts_element_index_counter += 1;
                            }
                        } else {
                            vec_of_hashmap_parts[vec_of_hashmap_parts_element_index_counter]
                                .insert(element.0, element.1);
                        }
                    } else if (vec_of_hashmap_parts.len() - 1)
                        != even_vec_of_hashmap_parts_element_index_counter
                    {
                        even_vec_of_hashmap_parts_element_index_counter += 1;
                        vec_of_hashmap_parts[even_vec_of_hashmap_parts_element_index_counter]
                            .insert(element.0, element.1);
                    } else {
                        vec_of_hashmap_parts[even_vec_of_hashmap_parts_element_index_counter]
                            .insert(element.0, element.1);
                        even_vec_of_hashmap_parts_element_index_counter = 0;
                    }
                }
            } else {
                for (element_index, element) in vec_of_links.into_iter().enumerate() {
                    vec_of_hashmap_parts[element_index].insert(element.0, element.1);
                }
            }
            if enable_prints {
                println!(
                    "thread::spawn for each provider must be done {:#?} times...",
                    vec_of_hashmap_parts.len()
                );
            }
            let not_ready_processed_posts = Arc::new(Mutex::new(Vec::with_capacity(links_len)));
            let mut threads_vector = Vec::with_capacity(vec_of_hashmap_parts.len());
            for element in &mut vec_of_hashmap_parts.into_iter() {
                let not_ready_processed_posts_handle = Arc::clone(&not_ready_processed_posts);
                let thread = thread::spawn(move || {
                    let unfiltered_posts_hashmap_after_fetch_and_parse =
                        twitter_fetch_and_parse_xml(
                            enable_prints,
                            enable_error_prints,
                            enable_time_measurement,
                            element.clone(),
                            ProviderKind::Twitter,
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
            let processed_posts = &*not_ready_processed_posts.lock().unwrap();
            let unfiltered_posts_hashmap_after_fetch_and_parse_len_counter = processed_posts.len();
            let (unhandled_success_handled_success_are_there_items_yep_posts, some_error_posts) =
                twitter_filter_fetched_and_parsed_posts(processed_posts.to_vec());
            if unhandled_success_handled_success_are_there_items_yep_posts.is_empty() {
                if enable_warning_prints {
                    print_warning_orange(
                        file!().to_string(),
                        line!().to_string(),
                        "unhandled_success_handled_success_are_there_items_yep_posts is EMPTY!!!"
                            .to_string(),
                    );
                }
                false
            } else if unhandled_success_handled_success_are_there_items_yep_posts.len()
                != unfiltered_posts_hashmap_after_fetch_and_parse_len_counter
            {
                let warning_message = format!(
                    "some_error_posts.len {} of {}",
                    some_error_posts.len(),
                    unhandled_success_handled_success_are_there_items_yep_posts.len()
                );
                print_warning_orange(file!().to_string(), line!().to_string(), warning_message);
                let wrong_cases_thread = thread::spawn(move || {
                    if enable_prints {
                        let message = format!(
                                        "(partially)succesfully_fetched_and_parsed_posts {} out of {} for {:#?}, allocated: {} byte/bytes",
                                        unhandled_success_handled_success_are_there_items_yep_posts.len(),
                                        unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
                                        provider_kind,
                                        mem::size_of_val(&unhandled_success_handled_success_are_there_items_yep_posts)
                                    );
                        print_partial_success_cyan(
                            file!().to_string(),
                            line!().to_string(),
                            message,
                        );
                    }
                    if enable_cleaning_logs_directory {
                        let path =
                            format!("logs/{}/{:?}", WARNING_LOGS_DIRECTORY_NAME, provider_kind);
                        if Path::new(&path).is_dir() {
                            let result_of_recursively_removing_warning_logs_directory =
                                fs::remove_dir_all(&path);
                            match result_of_recursively_removing_warning_logs_directory {
                                Ok(_) => {
                                    if enable_prints {
                                        println!("folder {} has been deleted", &path);
                                    }
                                }
                                Err(e) => {
                                    if enable_error_prints {
                                        let error_message = format!(
                                            "delete folder problem{} {}",
                                            &path,
                                            e.to_string()
                                        );
                                        print_error_red(
                                            file!().to_string(),
                                            line!().to_string(),
                                            error_message,
                                        )
                                    }
                                }
                            }
                        }
                    }
                    block_on(twitter_async_write_fetch_error_logs_into_files_wrapper(
                        provider_kind,
                        enable_prints,
                        // enable_warning_prints: bool,
                        enable_error_prints,
                        enable_time_measurement,
                        some_error_posts,
                    ));
                });
                wrong_cases_thread.join().unwrap();
                //todo: cast to common post type
                true
            } else {
                let message = format!(
                    "succesfully_fetched_and_parsed_posts {} out of {} for {:#?}, allocated: {} byte/bytes",
                    unhandled_success_handled_success_are_there_items_yep_posts.len(),
                    unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
                    provider_kind,
                    mem::size_of_val(&unhandled_success_handled_success_are_there_items_yep_posts)
                );
                if enable_prints {
                    print_success_green(file!().to_string(), line!().to_string(), message);
                }
                //todo: cast to common post type
                true
            }
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
        false
    }
}
