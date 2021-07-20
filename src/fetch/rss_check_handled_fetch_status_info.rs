use std::time::Instant;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_parse_string_into_struct::rss_parse_string_into_struct;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

#[allow(clippy::clippy::too_many_arguments)]
pub fn rss_check_handled_fetch_status_info(
    handled_fetch_status_info: HandledFetchStatusInfo,
    fetch_result_string: String,
    time: Instant,
    value: &str,
    enable_error_prints: bool,
    enable_time_measurement: bool,
    provider_kind: ProviderKind,
) -> (CommonRssPostStruct, HandledFetchStatusInfo, AreThereItems) {
    //todo: change order
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
            let provider_kind_clone_for_prints = provider_kind.clone();
            let (rxiv_post_struct_handle, are_there_items_handle) = rss_parse_string_into_struct(
                fetch_result_string,
                &value,
                enable_error_prints,
                provider_kind,
            );
            rxiv_post_struct_wrapper_handle = rxiv_post_struct_handle;
            are_there_items_wrapper_handle = are_there_items_handle;
            print_colorful_message(
                Some(&provider_kind_clone_for_prints),
                PrintType::TimeMeasurement,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "parse in {}.{}ms abs, rel {}.{}ms",
                    time.elapsed().as_secs(),
                    time.elapsed().as_millis(),
                    since_fetch.elapsed().as_secs(),
                    since_fetch.elapsed().as_millis(),
                ),
            );
        }
    }
    // println!(
    //     "rxiv_post_struct_wrapper_handleffff{:#?}",
    //     rxiv_post_struct_wrapper_handle
    // );
    (
        rxiv_post_struct_wrapper_handle,
        value3,
        are_there_items_wrapper_handle,
    )
}
