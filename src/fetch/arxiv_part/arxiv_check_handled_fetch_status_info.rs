use std::time::Instant;

use super::arxiv_parse_string_into_struct::arxiv_parse_string_into_struct;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rxiv_structures::RxivPostStruct;

pub fn check_handled_fetch_status_info(
    handled_fetch_status_info: HandledFetchStatusInfo,
    fetch_tuple_result_string: String,
    time: Instant,
    key: &str,
    value: &str,
) -> (HandledFetchStatusInfo, RxivPostStruct, AreThereItems) {
    let value3: HandledFetchStatusInfo;
    let mut arxiv_post_struct_wrapper_handle: RxivPostStruct = RxivPostStruct::new();
    let mut are_there_items_wrapper_handle: AreThereItems = AreThereItems::Initialized;
    match handled_fetch_status_info {
        HandledFetchStatusInfo::Initialized => {
            value3 = HandledFetchStatusInfo::Initialized;
        }
        HandledFetchStatusInfo::ResToTextError(res_to_text_string_error) => {
            value3 = HandledFetchStatusInfo::ResToTextError(res_to_text_string_error);
        }
        HandledFetchStatusInfo::ResStatusError(res_error_code) => {
            value3 = HandledFetchStatusInfo::ResStatusError(res_error_code);
        }
        HandledFetchStatusInfo::Success => {
            let since_fetch = Instant::now();
            value3 = HandledFetchStatusInfo::Success;
            let (arxiv_post_struct_handle, are_there_items_handle) =
                arxiv_parse_string_into_struct(fetch_tuple_result_string, key, &value);
            arxiv_post_struct_wrapper_handle = arxiv_post_struct_handle;
            are_there_items_wrapper_handle = are_there_items_handle;
            if ENABLE_PRINTS_ARXIV {
                println!(
                    "parse in {}.{}ms abs, rel {}.{}ms for {}",
                    time.elapsed().as_secs(),
                    time.elapsed().as_millis(),
                    since_fetch.elapsed().as_secs(),
                    since_fetch.elapsed().as_millis(),
                    key
                );
            }
        }
    }
    (
        value3,
        arxiv_post_struct_wrapper_handle,
        are_there_items_wrapper_handle,
    )
}
