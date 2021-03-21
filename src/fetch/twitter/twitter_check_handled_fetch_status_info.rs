use std::time::Instant;

use crate::fetch::provider_kind_enum::ProviderKind;
use crate::fetch::rxiv::metainfo_fetch_structures::AreThereItems;
use crate::fetch::rxiv::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::twitter::twitter_parse_string_into_struct::twitter_parse_string_into_struct;
use crate::fetch::twitter::twitter_structures::TwitterPostStruct;

#[allow(clippy::clippy::too_many_arguments)]
pub fn twitter_check_handled_fetch_status_info(
    handled_fetch_status_info: HandledFetchStatusInfo,
    fetch_result_string: String,
    time: Instant,
    key: &str,
    value: &str,
    enable_prints: bool,
    enable_error_prints: bool,
    provider_kind: ProviderKind,
) -> (HandledFetchStatusInfo, TwitterPostStruct, AreThereItems) {
    let value3: HandledFetchStatusInfo;
    let mut rxiv_post_struct_wrapper_handle: TwitterPostStruct = TwitterPostStruct::new();
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
            let (rxiv_post_struct_handle, are_there_items_handle) =
                twitter_parse_string_into_struct(
                    fetch_result_string,
                    key,
                    &value,
                    // enable_prints,
                    enable_error_prints,
                    provider_kind,
                );
            rxiv_post_struct_wrapper_handle = rxiv_post_struct_handle;
            are_there_items_wrapper_handle = are_there_items_handle;
            if enable_prints {
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
        rxiv_post_struct_wrapper_handle,
        are_there_items_wrapper_handle,
    )
}
