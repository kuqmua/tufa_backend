use colored::*;
use std::collections::HashMap;
use std::time::Instant;

use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rxiv_check_handled_fetch_status_info::rxiv_check_handled_fetch_status_info;
use crate::fetch::rxiv_fetch_link::rxiv_fetch_link;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::fetch::rxiv_structures::RxivPostStruct; //page instead of post wtf????

use crate::config::ENABLE_ERROR_PRINTS_MEDRXIV;
use crate::config::ENABLE_PRINTS_MEDRXIV;
use crate::get_group_names::get_medrxiv_links::get_medrxiv_links;
use crate::overriding::prints::print_error_red;

pub fn medrxiv_fetch_and_parse_xml() -> HashMap<
    String,
    (
        RxivPostStruct,
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
    ),
> {
    let time = Instant::now();
    let medrxiv_links_in_hash_map: HashMap<&str, &str> = get_medrxiv_links();
    let mut hashmap_to_return: HashMap<
        String,
        (
            RxivPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
        ),
    > = HashMap::new();
    for (key, value) in medrxiv_links_in_hash_map {
        let tuple = (
            RxivPostStruct::new(),
            value.to_string(),
            UnhandledFetchStatusInfo::Initialized,
            HandledFetchStatusInfo::Initialized,
            AreThereItems::Initialized,
        );
        hashmap_to_return.insert(key.to_string(), tuple);
    }
    if ENABLE_PRINTS_MEDRXIV {
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
                    ENABLE_PRINTS_MEDRXIV,
                    ENABLE_ERROR_PRINTS_MEDRXIV,
                );
                match fetch_result {
                    Ok(fetch_tuple_result) => {
                        value.2 = UnhandledFetchStatusInfo::Success;
                        let (
                            value3,
                            medrxiv_post_struct_wrapper_handle,
                            are_there_items_wrapper_handle,
                        ) = rxiv_check_handled_fetch_status_info(
                            fetch_tuple_result.1,
                            fetch_tuple_result.0,
                            time,
                            key,
                            &value.1,
                            ENABLE_PRINTS_MEDRXIV,
                            ENABLE_ERROR_PRINTS_MEDRXIV,
                            RxivKind::Medrxiv,
                        );
                        value.3 = value3;
                        value.0 = medrxiv_post_struct_wrapper_handle;
                        value.4 = are_there_items_wrapper_handle;
                    }
                    Err(e) => {
                        value.2 = UnhandledFetchStatusInfo::Failure(e.to_string()); // add e
                        if ENABLE_ERROR_PRINTS_MEDRXIV {
                            print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                        }
                    }
                }
            });
        }
    });
    match crossbeam_result {
        Ok(_) => {
            if ENABLE_PRINTS_MEDRXIV {
                println!("crossbeam_result is ok",)
            }
        }
        Err(e) => {
            if ENABLE_ERROR_PRINTS_MEDRXIV {
                eprintln!(
                    "crossbeam_result is not ok, file: {}, line {}\n {:#?}",
                    file!().to_string().red().bold(),
                    line!().to_string().red().bold(),
                    e
                )
            }
        }
    }
    // println!("medrxiv_sections_links {:#?}", hashmap_to_return);
    hashmap_to_return
}
