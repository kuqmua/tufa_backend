use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::helpers::fetch::blocking_fetch_link::blocking_fetch_link;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::fetch::rss_metainfo_fetch_structures::RssFetchLinkError;
use crate::fetch::rss_parse_string_into_struct::rss_parse_string_into_struct;
use crate::fetch::rss_handle_error_status_code::handle_error_status_code;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_fetch_and_parse_provider_data(
    links: Vec<String>,
    pk: ProviderKind,
) -> Vec<
    Result<
        Result<CommonRssPostStruct, (NoItemsError, String)>,
        (String, ProviderKind, RssFetchLinkError),
    >,
> {
    //RssFetchLinkError
    let time = Instant::now();
    let hashmap_to_return = Arc::new(Mutex::new(Vec::<
        Result<
            Result<CommonRssPostStruct, (NoItemsError, String)>,
            (String, ProviderKind, RssFetchLinkError),
        >,
    >::with_capacity(links.len())));
    let mut thread_vector = Vec::with_capacity(links.len());
    for link in &mut links.into_iter() {
        let hashmap_to_return_handle = Arc::clone(&hashmap_to_return);
        let pk_clone = pk;
        let handle = thread::spawn(move || {
            let fetch_result = blocking_fetch_link(&link, time);
            match fetch_result {
                Ok(response_text) => {
                    match rss_parse_string_into_struct(response_text, &link, pk) {
                        Ok(post_struct) => {
                            let mut hashmap_to_return_handle_locked =
                                hashmap_to_return_handle.lock().unwrap();
                            hashmap_to_return_handle_locked.push(Ok(Ok(post_struct)))
                        }
                        Err(e) => {
                            let mut hashmap_to_return_handle_locked =
                                hashmap_to_return_handle.lock().unwrap();
                            hashmap_to_return_handle_locked.push(Ok(Err((e, link))))
                        }
                    }
                }
                Err(e) => {
                    if let RssFetchLinkError::StatusCode(status_code) = e {
                        handle_error_status_code(status_code, &link);
                    }
                    print_colorful_message(
                        Some(&pk_clone),
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!("link: {} RssFetchLinkError {:#?}", link, e),
                    );
                    let mut hashmap_to_return_handle_locked =
                        hashmap_to_return_handle.lock().unwrap();
                    hashmap_to_return_handle_locked.push(Err((link, pk_clone, e)));
                }
            }
        });
        thread_vector.push(handle);
    }
    for thread in thread_vector {
        thread.join().unwrap();
    }
    let hashmap_to_return_done = hashmap_to_return.lock().unwrap().drain(..).collect();
    hashmap_to_return_done
}
