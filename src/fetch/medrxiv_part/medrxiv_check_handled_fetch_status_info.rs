use std::time::Instant;

use super::medrxiv_metainfo_structures::AreThereItems;
use super::medrxiv_metainfo_structures::HandledFetchStatusInfo;
use super::medrxiv_parse_string_into_struct::medrxiv_parse_string_into_struct;
use super::medrxiv_structures::MedrxivPageStruct;
use crate::config::ENABLE_PRINTS_MEDRXIV;

pub fn check_handled_fetch_status_info(
    handled_fetch_status_info: HandledFetchStatusInfo,
    fetch_tuple_result_string: String,
    time: Instant,
    key: &str,
    value: &str,
) -> (HandledFetchStatusInfo, MedrxivPageStruct, AreThereItems) {
    let value3: HandledFetchStatusInfo;
    let mut medrxiv_post_struct_wrapper_handle: MedrxivPageStruct = MedrxivPageStruct::new();
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
            let (medrxiv_post_struct_handle, are_there_items_handle) =
                medrxiv_parse_string_into_struct(fetch_tuple_result_string, key, &value);
            medrxiv_post_struct_wrapper_handle = medrxiv_post_struct_handle;
            are_there_items_wrapper_handle = are_there_items_handle;
            if ENABLE_PRINTS_MEDRXIV {
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
        medrxiv_post_struct_wrapper_handle,
        are_there_items_wrapper_handle,
    )
}
