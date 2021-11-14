// use std::time::Instant;

// use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
// use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
// use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
// use crate::fetch::rss_parse_string_into_struct::rss_parse_string_into_struct;
// use crate::providers::provider_kind_enum::ProviderKind;

// use crate::prints::print_colorful_message::print_colorful_message;
// use crate::prints::print_type_enum::PrintType;

// #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
// pub fn rss_check_handled_fetch_status_info(
//     response_text: String,
//     time: Instant,
//     value: &str,
//     provider_kind: ProviderKind,
// ) -> (CommonRssPostStruct, ) {
//     let since_fetch = Instant::now();

//     print_colorful_message(
//         Some(&provider_kind),
//         PrintType::TimeMeasurement,
//         file!().to_string(),
//         line!().to_string(),
//         format!(
//             "parse in {}.{}ms abs, rel {}.{}ms",
//             time.elapsed().as_secs(),
//             time.elapsed().as_millis(),
//             since_fetch.elapsed().as_secs(),
//             since_fetch.elapsed().as_millis(),
//         ),
//     );
//     (rxiv_post_struct_handle, are_there_items_handle)
// }
