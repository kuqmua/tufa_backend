extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::fetch::rss_async_write_fetch_error_logs_into_files_wrapper::rss_async_write_fetch_error_logs_into_files_wrapper;
use crate::fetch::rss_filter_fetched_and_parsed_posts::rss_filter_fetched_and_parsed_posts;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
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

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

#[allow(clippy::clippy::too_many_arguments)]
pub fn handle_unfiltered_posts(
    unfiltered_posts_hashmap_after_fetch_and_parse: Vec<(
        String,
        (
            CommonRssPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
        ),
    )>,
    provider_kind: &'static ProviderKind,
    enable_prints: bool,
    enable_warning_prints: bool,
    enable_error_prints: bool,
    enable_cleaning_logs_directory: bool,
    enable_time_measurement: bool,
    warning_logs_directory_name: &'static str,
) -> Option<HashMap<String, CommonRssPostStruct>> {
    let unfiltered_posts_hashmap_after_fetch_and_parse_len_counter =
        unfiltered_posts_hashmap_after_fetch_and_parse.len();
    let (unhandled_success_handled_success_are_there_items_yep_posts, some_error_posts) =
        rss_filter_fetched_and_parsed_posts(
            unfiltered_posts_hashmap_after_fetch_and_parse.to_vec(),
            provider_kind,
        );
    if unhandled_success_handled_success_are_there_items_yep_posts.is_empty() {
        if enable_warning_prints {
            print_warning_orange(
                file!().to_string(),
                line!().to_string(),
                "unhandled_success_handled_success_are_there_items_yep_posts is EMPTY!!!"
                    .to_string(),
            );
        }
        None
    } else if unhandled_success_handled_success_are_there_items_yep_posts.len()
        != unfiltered_posts_hashmap_after_fetch_and_parse_len_counter
    {
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
        let wrong_cases_thread = thread::spawn(move || {
            if enable_cleaning_logs_directory {
                let path = format!("logs/{}/{:?}", warning_logs_directory_name, provider_kind);
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
                                let error_message =
                                    format!("delete folder problem{} {}", &path, e.to_string());
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
            block_on(rss_async_write_fetch_error_logs_into_files_wrapper(
                provider_kind,
                enable_prints,
                enable_error_prints,
                enable_time_measurement,
                some_error_posts,
                warning_logs_directory_name,
            ));
        });
        wrong_cases_thread.join().unwrap();
        Some(unhandled_success_handled_success_are_there_items_yep_posts)
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
        Some(unhandled_success_handled_success_are_there_items_yep_posts)
    }
}
