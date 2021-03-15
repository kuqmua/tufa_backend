extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use std::thread;

use crate::check_net::check_link::check_link;
use crate::config::WARNING_LOGS_DIRECTORY_NAME;
use crate::fetch::rxiv_fetch_and_parse_xml::rxiv_fetch_and_parse_xml;
use crate::fetch::rxiv_filter_fetched_and_parsed_posts::rxiv_filter_fetched_and_parsed_posts;
use crate::fetch::rxiv_handle_errors_arrays::rxiv_handle_errors_arrays;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_partial_success_cyan;
use crate::overriding::prints::print_success_green;
use crate::overriding::prints::print_warning_orange;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn rxiv_part(
    links: HashMap<&'static str, &'static str>,
    enable_cleaning_logs_directory: bool,
    enable_prints: bool,
    enable_warning_prints: bool,
    enable_error_prints: bool,
    rxiv_url: &str,
    rxiv_kind: RxivKind,
) -> bool {
    if check_link(rxiv_url).0 {
        if enable_prints {
            println!("i can reach {}", rxiv_url)
        };
        let rxiv_kind_clone_for_debug_purposes = rxiv_kind.clone(); //only for debug
        let unfiltered_posts_hashmap_after_fetch_and_parse =
            rxiv_fetch_and_parse_xml(enable_prints, enable_error_prints, links, rxiv_kind.clone()); //rxiv_kind ниже еще используется
        let unfiltered_posts_hashmap_after_fetch_and_parse_len_counter =
            unfiltered_posts_hashmap_after_fetch_and_parse.len();
        let (
            //все отсальное херачить в отдельный поток кроме первого массива
            unhandled_success_handled_success_are_there_items_yep_posts,
            some_error_posts,
        ) = rxiv_filter_fetched_and_parsed_posts(unfiltered_posts_hashmap_after_fetch_and_parse);
        //переписать логику фильтрации выделяя тут только нужную часть//перенести в отдельный поток остальное
        let mut wrong_cases_thread = vec![];
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
            wrong_cases_thread.push(thread::spawn(move || {
                if enable_prints {
                    let message = format!(
                        "(partially)succesfully_fetched_and_parsed_posts {} out of {} for {:#?}",
                        unhandled_success_handled_success_are_there_items_yep_posts.len(),
                        unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
                        rxiv_kind_clone_for_debug_purposes
                    );
                    print_partial_success_cyan(file!().to_string(), line!().to_string(), message);
                }
                if enable_cleaning_logs_directory {
                    let path = format!("logs/{}/{:?}", WARNING_LOGS_DIRECTORY_NAME, rxiv_kind);
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
                rxiv_handle_errors_arrays(
                    rxiv_kind,
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
                rxiv_kind_clone_for_debug_purposes
            );
            print_success_green(file!().to_string(), line!().to_string(), message);
            // true
        }
        for i in wrong_cases_thread {
            i.join().unwrap();
        }
        true
    } else {
        if enable_error_prints {
            let error_message = format!("i cannot reach {}", rxiv_url);
            print_error_red(file!().to_string(), line!().to_string(), error_message);
            println!();
        };
        false
    }
}
