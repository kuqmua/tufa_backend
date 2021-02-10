use colored::*;
use std::collections::HashMap;
use std::time::Instant;

use super::arxiv_check_handled_fetch_status_info::check_handled_fetch_status_info;
use super::arxiv_structures::ArxivPostStruct;
use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rxiv_fetch_link::rxiv_fetch_link;

use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::overriding::prints::print_error_red;

pub fn arxiv_fetch_and_parse_xml() -> HashMap<
    String,
    (
        ArxivPostStruct,
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
    ),
> {
    let time = Instant::now();
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links();
    let mut hashmap_to_return: HashMap<
        String,
        (
            ArxivPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
        ),
    > = HashMap::new();
    for (key, value) in arxiv_links_in_hash_map {
        let tuple = (
            ArxivPostStruct::new(),
            value.to_string(),
            UnhandledFetchStatusInfo::Initialized,
            HandledFetchStatusInfo::Initialized,
            AreThereItems::Initialized,
        );
        hashmap_to_return.insert(key.to_string(), tuple);
    }
    if ENABLE_PRINTS_ARXIV {
        println!(
            "hashmap init in {}.{}ms",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
        );
    };
    let crossbeam_result = crossbeam::scope(|scope| {
        for (key, value) in &mut hashmap_to_return {
            scope.spawn(move |_| {
                let fetch_result = rxiv_fetch_link(
                    &value.1,
                    key,
                    time,
                    ENABLE_PRINTS_ARXIV,
                    ENABLE_ERROR_PRINTS_ARXIV,
                );
                match fetch_result {
                    Ok(fetch_tuple_result) => {
                        value.2 = UnhandledFetchStatusInfo::Success;
                        let (
                            value3,
                            arxiv_post_struct_wrapper_handle,
                            are_there_items_wrapper_handle,
                        ) = check_handled_fetch_status_info(
                            fetch_tuple_result.1,
                            fetch_tuple_result.0,
                            time,
                            key,
                            &value.1,
                        );
                        value.3 = value3;
                        value.0 = arxiv_post_struct_wrapper_handle;
                        value.4 = are_there_items_wrapper_handle;
                    }
                    Err(e) => {
                        value.2 = UnhandledFetchStatusInfo::Failure(e.to_string()); // add e
                        if ENABLE_ERROR_PRINTS_ARXIV {
                            print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                        }
                    }
                }
            });
        }
    });
    match crossbeam_result {
        Ok(_) => {
            if ENABLE_PRINTS_ARXIV {
                println!("crossbeam_result is ok",)
            }
        }
        Err(e) => {
            if ENABLE_ERROR_PRINTS_ARXIV {
                eprintln!(
                    "crossbeam_result is not ok, file: {}, line {}\n {:#?}",
                    file!().to_string().red().bold(),
                    line!().to_string().red().bold(),
                    e
                )
            }
        }
    }
    // println!("arxiv_sections_links {:#?}", hashmap_to_return);
    hashmap_to_return
}
