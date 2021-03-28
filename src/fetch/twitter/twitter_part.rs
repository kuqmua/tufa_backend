extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use std::thread;

use crate::check_net::check_link::check_link;
use crate::config::WARNING_LOGS_DIRECTORY_NAME;
use crate::fetch::provider_kind_enum::ProviderKind;
use crate::fetch::rxiv::rxiv_filter_fetched_and_parsed_posts::rxiv_filter_fetched_and_parsed_posts;
use crate::fetch::rxiv::rxiv_handle_errors_arrays::rxiv_handle_errors_arrays;
use crate::fetch::twitter::twitter_check_available_providers::twitter_check_available_providers;
use crate::fetch::twitter::twitter_fetch_and_parse_xml::twitter_fetch_and_parse_xml;
use crate::get_group_names::get_twitter_links::get_twitter_links;
use crate::get_group_names::get_twitter_providers::get_twitter_providers;
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
    provider_url: &str,
    provider_kind: ProviderKind,
) -> bool {
    let twitter_providers_names: Vec<&str> = get_twitter_providers();
    let twitter_providers_names_length_for_debug = twitter_providers_names.len();
    let twitter_available_providers_links: Vec<String> = twitter_check_available_providers(
        enable_prints,
        enable_error_prints,
        twitter_providers_names,
    );
    if !twitter_available_providers_links.is_empty() {
        let links = get_twitter_links(&twitter_available_providers_links);
        if !links.is_empty() {
            // if enable_prints {
            //     println!(
            //         "twitter providers available {:#?} of {:#?}",
            //         twitter_available_providers_links.len(),
            //         twitter_providers_names_length_for_debug
            //     );
            //     println!(
            //         "links({}) for each provider ({}) links_for_each_provider {} links_to_remaind {}",
            //         links.len(),
            //         twitter_available_providers_links.len(),
            //         links.len() / twitter_available_providers_links.len(),
            //         links.len() % twitter_available_providers_links.len(),
            //     );
            // }

            let mut vec_of_links: Vec<(&str, String)> = Vec::with_capacity(links.len());
            let links_for_each_provider: usize =
                links.len() / twitter_available_providers_links.len();
            let even_links_length_size_for_remaind: usize =
                links_for_each_provider * twitter_available_providers_links.len();
            let mut vec_of_hashmap_parts: Vec<HashMap<&str, String>> = vec![
                HashMap::with_capacity(links_for_each_provider);
                twitter_available_providers_links.len()
            ];

            let mut vec_of_hashmap_parts_position: usize = 0;
            for (key, value) in links {
                vec_of_links.push((key, value));
            }
            let mut i_to_each: usize = 1;
            let mut counter: usize = 1;
            for element in vec_of_links {
                if counter < even_links_length_size_for_remaind {
                    if i_to_each <= links_for_each_provider {
                        vec_of_hashmap_parts[vec_of_hashmap_parts_position]
                            .insert(element.0, element.1);
                        i_to_each += 1;
                    }
                    if counter != 0 && counter % links_for_each_provider == 0 {
                        vec_of_hashmap_parts_position += 1;
                        i_to_each = 1;
                    }
                } else {
                    vec_of_hashmap_parts[vec_of_hashmap_parts_position]
                        .insert(element.0, element.1);
                }
                counter += 1;
            }
            ///
            let crossbeam_result = crossbeam::scope(|scope| {
                for element in &mut vec_of_hashmap_parts {
                    scope.spawn(move |_| {
                        // let provider_kind_clone_for_debug_purposes = provider_kind.clone(); //only for debug
                        let unfiltered_posts_hashmap_after_fetch_and_parse =
                            twitter_fetch_and_parse_xml(
                                enable_prints,
                                enable_error_prints,
                                element.clone(),
                                ProviderKind::Twitter,
                            );
                        println!(
                            "unfiltered_posts_hashmap_after_fetch_and_parse.len() {:#?}",
                            unfiltered_posts_hashmap_after_fetch_and_parse.len()
                        )
                    });
                }
            });
            match crossbeam_result {
                Ok(_) => {
                    if enable_prints {
                        println!("twitter_part is ok")
                    }
                }
                Err(error) => {
                    if enable_error_prints {
                        let error_message = format!("twitter_part is not ok: {:#?}", error);
                        print_error_red(file!().to_string(), line!().to_string(), error_message);
                    }
                }
            }
            ///
            // // provider_kind ниже еще используется
            //             let unfiltered_posts_hashmap_after_fetch_and_parse_len_counter =
            //                 unfiltered_posts_hashmap_after_fetch_and_parse.len();
            //             let (
            //                 //все отсальное херачить в отдельный поток кроме первого массива
            //                 unhandled_success_handled_success_are_there_items_yep_posts,
            //                 some_error_posts,
            //             ) = rxiv_filter_fetched_and_parsed_posts(
            //                 unfiltered_posts_hashmap_after_fetch_and_parse,
            //             );
            // //переписать логику фильтрации выделяя тут только нужную часть//перенести в отдельный поток остальное
            // let mut wrong_cases_thread = vec![];
            // if unhandled_success_handled_success_are_there_items_yep_posts.is_empty() {
            //     if enable_warning_prints {
            //         print_warning_orange(
            //             file!().to_string(),
            //             line!().to_string(),
            //             "unhandled_success_handled_success_are_there_items_yep_posts is EMPTY!!!"
            //                 .to_string(),
            //         );
            //     }
            //     // false
            // } else if unhandled_success_handled_success_are_there_items_yep_posts.len()
            //     != unfiltered_posts_hashmap_after_fetch_and_parse_len_counter
            // {
            //     wrong_cases_thread.push(thread::spawn(move || {
            //         if enable_prints {
            //             let message = format!(
            //                 "(partially)succesfully_fetched_and_parsed_posts {} out of {} for {:#?}",
            //                 unhandled_success_handled_success_are_there_items_yep_posts.len(),
            //                 unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
            //                 provider_kind_clone_for_debug_purposes
            //             );
            //             print_partial_success_cyan(file!().to_string(), line!().to_string(), message);
            //         }
            //         if enable_cleaning_logs_directory {
            //             let path = format!("logs/{}/{:?}", WARNING_LOGS_DIRECTORY_NAME, provider_kind);
            //             if Path::new(&path).is_dir() {
            //                 let result_of_recursively_removing_warning_logs_directory =
            //                     fs::remove_dir_all(&path);
            //                 match result_of_recursively_removing_warning_logs_directory {
            //                     Ok(_) => {
            //                         if enable_prints {
            //                             println!("папка {} удалена", &path);
            //                         }
            //                     }
            //                     Err(e) => {
            //                         if enable_error_prints {
            //                             let message = format!(
            //                                 "проблема с удалением папки {} {}",
            //                                 &path,
            //                                 e.to_string()
            //                             );
            //                             print_error_red(
            //                                 file!().to_string(),
            //                                 line!().to_string(),
            //                                 message,
            //                             )
            //                         }
            //                     }
            //                 }
            //             }
            //         }
            //         rxiv_handle_errors_arrays(
            //             provider_kind,
            //             enable_prints,
            //             // enable_warning_prints,
            //             enable_error_prints,
            //             some_error_posts,
            //         );
            //     }));
            //     // true
            // } else {
            //     let message = format!(
            //         "succesfully_fetched_and_parsed_posts {} out of {} for {:#?}",
            //         unhandled_success_handled_success_are_there_items_yep_posts.len(),
            //         unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
            //         provider_kind_clone_for_debug_purposes
            //     );
            //     print_success_green(file!().to_string(), line!().to_string(), message);
            //     // true
            // }
            // for i in wrong_cases_thread {
            //     i.join().unwrap();
            // }
            true
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
