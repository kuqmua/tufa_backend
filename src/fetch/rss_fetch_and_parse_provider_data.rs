use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_fetch_link::rss_fetch_link;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use super::rss_parse_string_into_struct::rss_parse_string_into_struct;

pub fn rss_fetch_and_parse_provider_data(
    links: Vec<String>,
    provider_kind: ProviderKind,
) -> Vec<Result<Result<(CommonRssPostStruct, String), NoItemsError>, String>> {
    //RssFetchLinkError
    let time = Instant::now();
    let hashmap_to_return = Arc::new(Mutex::new(Vec::<
        Result<Result<(CommonRssPostStruct, String), NoItemsError>, String>,
    >::with_capacity(links.len())));
    let mut thread_vector = Vec::with_capacity(links.len());
    for (element_index, link) in &mut links.into_iter().enumerate() {
        let hashmap_to_return_handle = Arc::clone(&hashmap_to_return);
        let provider_kind_clone = provider_kind;
        let handle = thread::spawn(move || {
            let fetch_result = rss_fetch_link(&link, time);
            match fetch_result {
                Ok(response_text) => {
                    match rss_parse_string_into_struct(response_text, &link, provider_kind) {
                        Ok(post_struct) => {
                            let mut hashmap_to_return_handle_locked =
                                hashmap_to_return_handle.lock().unwrap();
                            hashmap_to_return_handle_locked.push(Ok(Ok((post_struct, link))))
                        }
                        Err(e) => {
                            let mut hashmap_to_return_handle_locked =
                                hashmap_to_return_handle.lock().unwrap();
                            hashmap_to_return_handle_locked.push(Ok(Err(e)))
                        }
                    }
                }
                Err(e) => {
                    print_colorful_message(
                        Some(&provider_kind_clone),
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!("RssFetchLinkError {:#?}", e),
                    );
                    let mut hashmap_to_return_handle_locked =
                        hashmap_to_return_handle.lock().unwrap();
                    hashmap_to_return_handle_locked[element_index] = Err(e.to_string());
                    //it must not be a string. i dont know how to clone custom error(enum) RssFetchLinkError
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
