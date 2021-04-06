extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use std::thread;

use crate::check_net::check_link::check_link;
use crate::config::WARNING_LOGS_DIRECTORY_NAME;
use crate::fetch::provider_kind_enum::ProviderKind;
use crate::fetch::twitter::twitter_check_available_providers::twitter_check_available_providers;
use crate::fetch::twitter::twitter_fetch_and_parse_xml::twitter_fetch_and_parse_xml;
use crate::fetch::twitter::twitter_filter_fetched_and_parsed_posts::twitter_filter_fetched_and_parsed_posts;
use crate::fetch::twitter::twitter_handle_errors_arrays::twitter_handle_errors_arrays;
use crate::get_group_names::get_twitter_subs::get_twitter_subs;
use crate::get_group_names::get_twitter_providers_names::get_twitter_providers_names;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_partial_success_cyan;
use crate::overriding::prints::print_success_green;
use crate::overriding::prints::print_warning_orange;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn twitter_part(
    // links: HashMap<&'static str, String>,
    enable_cleaning_logs_directory: bool,
    enable_prints: bool,
    enable_warning_prints: bool,
    enable_error_prints: bool,
    enable_time_measurement: bool,
    provider_url: &str,
    provider_kind: &'static ProviderKind,
) -> bool {
    let twitter_providers_names: Vec<&str> = get_twitter_providers_names();
    let twitter_providers_names_length_for_debug = twitter_providers_names.len();
    let twitter_available_providers_links: Vec<&str> = twitter_check_available_providers(
        //String
        //отсюда идет с http b rss
        enable_prints,
        enable_error_prints,
        twitter_providers_names,
    );
    if !twitter_available_providers_links.is_empty() {
        let links = get_twitter_subs(twitter_available_providers_links.clone());
        if !links.is_empty() {
            if enable_prints {
                println!(
                    "twitter providers available {:#?} of {:#?}",
                    twitter_available_providers_links.len(),
                    twitter_providers_names_length_for_debug
                );
                println!(
                    "links({}) for each provider ({}) links_for_each_provider {} links_to_remaind {}",
                    links.len(),
                    twitter_available_providers_links.len(),
                    links.len() / twitter_available_providers_links.len(),
                    links.len() % twitter_available_providers_links.len(),
                );
            }
            let links_len = links.len();
            let links_for_each_provider: usize;
            let is_links_len_more_than_twitter_available_providers_links_len = links_len > twitter_available_providers_links.len();
            let vec_of_hashmap_parts_len: usize;
            if is_links_len_more_than_twitter_available_providers_links_len {
                links_for_each_provider = links_len / twitter_available_providers_links.len();
                vec_of_hashmap_parts_len = twitter_available_providers_links.len();
            } else {
                links_for_each_provider = links_len;
                vec_of_hashmap_parts_len = links_len;
            }
            
            let mut vec_of_hashmap_parts: Vec<HashMap<&str, String>> = vec![
                HashMap::with_capacity(links_for_each_provider);
                vec_of_hashmap_parts_len
            ];
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
                            vec_of_hashmap_parts[even_vec_of_hashmap_parts_element_index_counter]
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
            }
            else{
                for (element_index, element) in vec_of_links.into_iter().enumerate() {
                    vec_of_hashmap_parts[element_index].insert(element.0, element.1);
                }
            }
            
            // println!("vec_of_hashmap_parts {:#?}", vec_of_hashmap_parts);
            let crossbeam_result = crossbeam::scope(|scope| {
                for element in &mut vec_of_hashmap_parts {
                    scope.spawn(move |_| {
                        let unfiltered_posts_hashmap_after_fetch_and_parse =
                            twitter_fetch_and_parse_xml(
                                enable_prints,
                                enable_error_prints,
                                enable_time_measurement,
                                element.clone(),
                                ProviderKind::Twitter,
                            );
                        println!(
                            "unfiltered_posts_hashmap_after_fetch_and_parse.len() {:#?}",
                            unfiltered_posts_hashmap_after_fetch_and_parse.len()
                        );
                        // provider_kind ниже еще используется
                        let unfiltered_posts_hashmap_after_fetch_and_parse_len_counter =
                            unfiltered_posts_hashmap_after_fetch_and_parse.len();
                        let (
                            //все отсальное херачить в отдельный поток кроме первого массива
                            unhandled_success_handled_success_are_there_items_yep_posts,
                            some_error_posts,
                        ) = twitter_filter_fetched_and_parsed_posts(
                            unfiltered_posts_hashmap_after_fetch_and_parse,
                        );
                        println!("unhandled_success_handled_success_are_there_items_yep_posts.len {}, some_error_posts.len {}", unhandled_success_handled_success_are_there_items_yep_posts.len(), some_error_posts.len());
                        // //переписать логику фильтрации выделяя тут только нужную часть//перенести в отдельный поток остальное
                        let mut wrong_cases_thread_vec = vec![];
                        
                        if unhandled_success_handled_success_are_there_items_yep_posts.is_empty() {
                            if enable_warning_prints {
                                print_warning_orange(
                                    file!().to_string(),
                                    line!().to_string(),
                                    "unhandled_success_handled_success_are_there_items_yep_posts is EMPTY!!!"
                                        .to_string(),
                                );
                            }
                            // false
                        } else if unhandled_success_handled_success_are_there_items_yep_posts.len()
                            != unfiltered_posts_hashmap_after_fetch_and_parse_len_counter
                        {
                            wrong_cases_thread_vec.push(thread::spawn(move || {
                                if enable_prints {
                                    let message = format!(
                                        "(partially)succesfully_fetched_and_parsed_posts {} out of {} for {:#?}",
                                        unhandled_success_handled_success_are_there_items_yep_posts.len(),
                                        unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
                                        provider_kind
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
                                twitter_handle_errors_arrays(
                                    provider_kind,
                                    enable_prints,
                                    // enable_warning_prints,
                                    enable_error_prints,
                                    some_error_posts,
                                );
                            }));
                            // true
                        } else {
                            let message = format!(
                                "succesfully_fetched_and_parsed_posts {} out of {} for {:#?}",
                                unhandled_success_handled_success_are_there_items_yep_posts.len(),
                                unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
                                provider_kind
                            );
                            print_success_green(file!().to_string(), line!().to_string(), message);
                        }
                        for i in wrong_cases_thread_vec {
                            i.join().unwrap();
                        }
                    });
                }
            });
            match crossbeam_result {
                Ok(_) => {
                    if enable_prints {
                        println!("twitter_part is ok")
                    }
                    true
                }
                Err(error) => {
                    if enable_error_prints {
                        let error_message = format!("twitter_part is not ok: {:#?}", error);
                        print_error_red(file!().to_string(), line!().to_string(), error_message);
                    }
                    false
                }
            }
        } else {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "twitter_links.is_empty".to_string(),
            );
            false
        }
    } else {
        false
    }
}
