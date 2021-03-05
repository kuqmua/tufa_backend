extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::check_net::check_link::check_link;
use crate::fetch::rxiv_fetch_and_parse_xml::rxiv_fetch_and_parse_xml;
use crate::fetch::rxiv_filter_fetched_and_parsed_posts::rxiv_filter_fetched_and_parsed_posts;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::overriding::prints::print_warning_orange;
use crate::overriding::prints::print_warning_yellow;
use std::collections::HashMap;
use std::{fs::File, io::Write};

pub fn rxiv_part(
    links: HashMap<&'static str, &'static str>,
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
            rxiv_fetch_and_parse_xml(enable_prints, enable_error_prints, links, rxiv_kind);
        let unfiltered_posts_hashmap_after_fetch_and_parse_len_counter =
            unfiltered_posts_hashmap_after_fetch_and_parse.len();
        let (
            unhandled_success_handled_success_are_there_items_yep_posts,
            unhandled_success_handled_success_are_there_items_initialized_posts,
            unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts,
            unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts,
            unhandled_success_handled_success_are_there_items_nope_no_tag_posts,
            unhandled_success_handled_initialized_posts,
            unhandled_success_handled_res_to_text_error_posts,
            unhandled_success_handled_res_status_error_posts,
            unhandled_initialized_posts,
            unhandled_failure_posts,
        ) = rxiv_filter_fetched_and_parsed_posts(unfiltered_posts_hashmap_after_fetch_and_parse);
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
        } else {
            if !unhandled_success_handled_success_are_there_items_initialized_posts.is_empty()
                && enable_warning_prints
            {
                let warning_message =
                    "unhandled_success_handled_success_are_there_items_initialized_posts.len() "
                        .to_string()
                        + &unhandled_success_handled_success_are_there_items_initialized_posts
                            .len()
                            .to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
            }
            if !unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts
                .is_empty()
                && enable_warning_prints
            {
                let warning_message = "unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts.len() ".to_string()
                    + &unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts.len().to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts
                .is_empty()
                && enable_warning_prints
            {
                let warning_message = "unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts.len() ".to_string()
                    + &unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts.len().to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_success_handled_success_are_there_items_nope_no_tag_posts.is_empty()
                && enable_warning_prints
            {
                let warning_message =
                    "unhandled_success_handled_success_are_there_items_nope_no_tag_posts.len() "
                        .to_string()
                        + &unhandled_success_handled_success_are_there_items_nope_no_tag_posts
                            .len()
                            .to_string();
                for (key, value) in
                    unhandled_success_handled_success_are_there_items_nope_no_tag_posts
                {
                    println!("HERE key {} value 0 {}", key, value.0);
                    let mut fileonos = File::create("logs/warning_logs/errorlogs.txt")
                        .expect("could not create file");
                    // writeln!(&mut fileonos, "{}", warning_message).unwrap();
                    let result_of_writing = fileonos.write(value.1.as_bytes()); //warning_message
                    match result_of_writing {
                        Ok(_) => println!("записано"),
                        Err(e) => println!("error {}", e),
                    }
                }

                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_success_handled_initialized_posts.is_empty() && enable_warning_prints {
                let warning_message = "unhandled_success_handled_initialized_posts.len() "
                    .to_string()
                    + &unhandled_success_handled_initialized_posts
                        .len()
                        .to_string();

                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_success_handled_res_to_text_error_posts.is_empty()
                && enable_warning_prints
            {
                let warning_message = "unhandled_success_handled_res_to_text_error_posts.len() "
                    .to_string()
                    + &unhandled_success_handled_res_to_text_error_posts
                        .len()
                        .to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_success_handled_res_status_error_posts.is_empty() && enable_warning_prints
            {
                let warning_message = "unhandled_success_handled_res_status_error_posts.len() "
                    .to_string()
                    + &unhandled_success_handled_res_status_error_posts
                        .len()
                        .to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_initialized_posts.is_empty() && enable_warning_prints {
                let warning_message = "unhandled_initialized_posts.len() ".to_string()
                    + &unhandled_initialized_posts.len().to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            if !unhandled_failure_posts.is_empty() && enable_warning_prints {
                let warning_message = "unhandled_failure_posts.len() ".to_string()
                    + &unhandled_failure_posts.len().to_string();
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
            }
            println!(
                "succesfully_fetched_and_parsed_posts {} out of {} for {:#?}",
                unhandled_success_handled_success_are_there_items_yep_posts.len(),
                unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
                rxiv_kind_clone_for_debug_purposes
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
