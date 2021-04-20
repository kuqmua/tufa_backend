use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::provider_kind_enum::ProviderKind;
// use crate::fetch::rxiv::rxiv_check_handled_fetch_status_info::rxiv_check_handled_fetch_status_info;
use crate::fetch::rxiv::rxiv_fetch_link::rxiv_fetch_link;
use crate::fetch::rxiv_structures::RxivPostStruct;
use crate::fetch::twitter::twitter_check_handled_fetch_status_info::twitter_check_handled_fetch_status_info;
use crate::overriding::prints::print_error_red;
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub fn rxiv_fetch_and_parse_xml(
    enable_prints: bool,
    enable_error_prints: bool,
    enable_time_measurement: bool,
    rxiv_links: HashMap<&'static str, std::string::String>,
    provider_kind: ProviderKind,
) -> Vec<(
    String,
    (
        RxivPostStruct,
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
    ),
)> {
    let time = Instant::now();
    let hashmap_to_return = Arc::new(Mutex::new(vec![
        (
            "".to_string(),
            (
                RxivPostStruct::new(),
                "".to_string(),
                UnhandledFetchStatusInfo::Initialized,
                HandledFetchStatusInfo::Initialized,
                AreThereItems::Initialized,
            )
        );
        rxiv_links.len()
    ]));
    if enable_time_measurement {
        println!(
            "hashmap init in {}.{}ms",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
        );
    };
    let mut handles = Vec::with_capacity(rxiv_links.len());

    for (element_index, (key, value)) in &mut rxiv_links.into_iter().enumerate() {
        let hashmap_handle = Arc::clone(&hashmap_to_return);
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
                    let mut hashmap_handle_locked = hashmap_handle.lock().unwrap();
                    hashmap_handle_locked[element_index].0 = key.to_string();
                    hashmap_handle_locked[element_index].1 .0 = rxiv_post_struct_wrapper_handle;
                    hashmap_handle_locked[element_index].1 .1 = value;
                    hashmap_handle_locked[element_index].1 .2 = UnhandledFetchStatusInfo::Success;
                    hashmap_handle_locked[element_index].1 .3 = value3;
                    hashmap_handle_locked[element_index].1 .4 = are_there_items_wrapper_handle;
                }
                Err(e) => {
                    let mut hashmap_handle_locked = hashmap_handle.lock().unwrap();
                    hashmap_handle_locked[element_index].1 .2 =
                        UnhandledFetchStatusInfo::Failure(e.to_string()); // add e
                    if enable_error_prints {
                        let concated_error =
                            "UnhandledFetchStatusInfo::Failure".to_string() + &e.to_string();
                        print_error_red(file!().to_string(), line!().to_string(), concated_error)
                    }
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let processed_posts = hashmap_to_return.lock().unwrap().to_vec();
    processed_posts
}
