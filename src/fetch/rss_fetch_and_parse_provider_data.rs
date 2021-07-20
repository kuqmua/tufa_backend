use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_check_handled_fetch_status_info::rss_check_handled_fetch_status_info;
use crate::fetch::rss_fetch_link::rss_fetch_link;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub fn rss_fetch_and_parse_provider_data(
    links: Vec<String>,
    provider_kind: ProviderKind,
) -> Vec<(
    CommonRssPostStruct,
    String,
    UnhandledFetchStatusInfo,
    HandledFetchStatusInfo,
    AreThereItems,
)> {
    let time = Instant::now();
    let hashmap_to_return = Arc::new(Mutex::new(vec![
        (
            CommonRssPostStruct::new(),
            "".to_string(),
            UnhandledFetchStatusInfo::Initialized,
            HandledFetchStatusInfo::Initialized,
            AreThereItems::Initialized,
        );
        links.len()
    ]));
    print_colorful_message(
        Some(&provider_kind),
        PrintType::TimeMeasurement,
        file!().to_string(),
        line!().to_string(),
        format!(
            "hashmap init in {}.{}ms",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
        ),
    );
    let mut thread_vector = Vec::with_capacity(links.len());
    for (element_index, link) in &mut links.into_iter().enumerate() {
        let hashmap_to_return_handle = Arc::clone(&hashmap_to_return);
        let provider_kind_clone = provider_kind.clone();
        let handle = thread::spawn(move || {
            let fetch_result = rss_fetch_link(&link, time);
            match fetch_result {
                Ok(fetch_tuple_result) => {
                    let (post_struct_wrapper_handle, value3, are_there_items_wrapper_handle) =
                        rss_check_handled_fetch_status_info(
                            fetch_tuple_result.1,
                            fetch_tuple_result.0,
                            time,
                            &link,
                            provider_kind_clone,
                        );
                    let mut hashmap_to_return_handle_locked =
                        hashmap_to_return_handle.lock().unwrap();
                    hashmap_to_return_handle_locked[element_index].0 = post_struct_wrapper_handle;
                    hashmap_to_return_handle_locked[element_index].1 = link;
                    hashmap_to_return_handle_locked[element_index].2 =
                        UnhandledFetchStatusInfo::Success;
                    hashmap_to_return_handle_locked[element_index].3 = value3;
                    hashmap_to_return_handle_locked[element_index].4 =
                        are_there_items_wrapper_handle;
                }
                Err(e) => {
                    let mut hashmap_to_return_handle_locked =
                        hashmap_to_return_handle.lock().unwrap();
                    hashmap_to_return_handle_locked[element_index].1 = link;
                    hashmap_to_return_handle_locked[element_index].2 =
                        UnhandledFetchStatusInfo::Failure(e.to_string());
                    print_colorful_message(
                        Some(&provider_kind_clone),
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!("UnhandledFetchStatusInfo::Failure {:#?}", &e.to_string()),
                    );
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
