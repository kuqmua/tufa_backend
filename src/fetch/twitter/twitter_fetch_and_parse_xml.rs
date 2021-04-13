use std::collections::HashMap;
use std::time::Instant;

use crate::fetch::provider_kind_enum::ProviderKind;
use crate::fetch::rxiv::metainfo_fetch_structures::AreThereItems;
use crate::fetch::rxiv::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rxiv::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rxiv::rxiv_fetch_link::rxiv_fetch_link;
use crate::fetch::twitter::twitter_check_handled_fetch_status_info::twitter_check_handled_fetch_status_info;
use crate::fetch::twitter::twitter_structures::TwitterPostStruct;
use crate::overriding::prints::print_error_red;

use std::sync::{Arc, Mutex};
use std::thread;

pub fn twitter_fetch_and_parse_xml(
    enable_prints: bool,
    enable_error_prints: bool,
    enable_time_measurement: bool,
    twitter_links: std::collections::HashMap<&'static str, std::string::String>,
    provider_kind: ProviderKind,
) -> Vec<(
    String,
    (
        TwitterPostStruct,
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    ),
)> {
    let time = Instant::now();
    let hashmap_to_return = Arc::new(Mutex::new(vec![
        (
            "".to_string(),
            (
                TwitterPostStruct::new(),
                "".to_string(),
                UnhandledFetchStatusInfo::Initialized,
                HandledFetchStatusInfo::Initialized,
                AreThereItems::Initialized,
                provider_kind.clone()
            )
        );
        twitter_links.len()
    ]));
    if enable_time_measurement {
        println!(
            "hashmap init in {}.{}ms",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
        );
    };
    let mut thread_vector = vec![];
    for (element_index, (key, value)) in &mut twitter_links.into_iter().enumerate() {
        let hashmap_to_return_handle = Arc::clone(&hashmap_to_return);
        let provider_kind_clone = provider_kind.clone();
        let handle = thread::spawn(move || {
            let fetch_result = rxiv_fetch_link(
                &value,
                key,
                time,
                enable_prints,
                enable_error_prints,
                enable_time_measurement,
            );
            match fetch_result {
                Ok(fetch_tuple_result) => {
                    let (value3, rxiv_post_struct_wrapper_handle, are_there_items_wrapper_handle) =
                        twitter_check_handled_fetch_status_info(
                            fetch_tuple_result.1,
                            fetch_tuple_result.0,
                            time,
                            key,
                            &value,
                            enable_prints,
                            enable_error_prints,
                            enable_time_measurement,
                            provider_kind_clone,
                        );
                    let mut hashmap_to_return_handle_locked =
                        hashmap_to_return_handle.lock().unwrap();
                    hashmap_to_return_handle_locked[element_index].0 = key.to_string();
                    hashmap_to_return_handle_locked[element_index].1 .0 =
                        rxiv_post_struct_wrapper_handle;
                    hashmap_to_return_handle_locked[element_index].1 .1 = value;
                    hashmap_to_return_handle_locked[element_index].1 .2 =
                        UnhandledFetchStatusInfo::Success;
                    hashmap_to_return_handle_locked[element_index].1 .3 = value3;
                    hashmap_to_return_handle_locked[element_index].1 .4 =
                        are_there_items_wrapper_handle;
                }
                Err(e) => {
                    if enable_error_prints {
                        let concated_error =
                            "UnhandledFetchStatusInfo::Failure".to_string() + &e.to_string();
                        print_error_red(file!().to_string(), line!().to_string(), concated_error)
                    }
                    let mut hashmap_to_return_handle_locked =
                        hashmap_to_return_handle.lock().unwrap();
                    hashmap_to_return_handle_locked[element_index].1 .2 =
                        UnhandledFetchStatusInfo::Failure(e.to_string());
                }
            }
        });
        thread_vector.push(handle);
    }
    for thread in thread_vector {
        thread.join().unwrap();
    }
    let hashmap_to_return_done = hashmap_to_return.lock().unwrap().to_vec();
    hashmap_to_return_done
}
