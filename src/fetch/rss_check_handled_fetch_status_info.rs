use std::time::Instant;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_parse_string_into_struct::rss_parse_string_into_struct;
use crate::fetch::rss_provider_kind_enum::ProviderKind;

#[allow(clippy::clippy::too_many_arguments)]
pub fn rss_check_handled_fetch_status_info(
    handled_fetch_status_info: HandledFetchStatusInfo,
    fetch_result_string: String,
    time: Instant,
    value: &str,
    enable_error_prints: bool,
    enable_time_measurement: bool,
    provider_kind: ProviderKind,
) -> (HandledFetchStatusInfo, CommonRssPostStruct, AreThereItems) {
    let value3: HandledFetchStatusInfo;
    let mut rxiv_post_struct_wrapper_handle: CommonRssPostStruct = CommonRssPostStruct::new();
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
            let (rxiv_post_struct_handle, are_there_items_handle) = rss_parse_string_into_struct(
                fetch_result_string,
                // key,
                &value,
                // enable_prints,
                enable_error_prints,
                provider_kind,
            );
            rxiv_post_struct_wrapper_handle = rxiv_post_struct_handle;
            are_there_items_wrapper_handle = are_there_items_handle;
            if enable_time_measurement {
                println!(
                    "parse in {}.{}ms abs, rel {}.{}ms",
                    time.elapsed().as_secs(),
                    time.elapsed().as_millis(),
                    since_fetch.elapsed().as_secs(),
                    since_fetch.elapsed().as_millis(),
                );
            }
        }
    }
    // println!(
    //     "rxiv_post_struct_wrapper_handleffff{:#?}",
    //     rxiv_post_struct_wrapper_handle
    // );
    (
        value3,
        rxiv_post_struct_wrapper_handle,
        are_there_items_wrapper_handle,
    )
}
