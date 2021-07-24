extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::fetch::rss_filter_fetched_and_parsed_posts::rss_filter_fetched_and_parsed_posts;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use std::mem;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[allow(clippy::clippy::too_many_arguments)]
pub fn rss_handle_unfiltered_posts(
    unfiltered_posts_hashmap_after_fetch_and_parse: Vec<(
        CommonRssPostStruct,
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
    )>,
    provider_kind: ProviderKind,
) -> (
    Option<Vec<CommonRssPostStruct>>,
    Option<
        Vec<(
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
            ProviderKind,
        )>,
    >,
) {
    let unfiltered_posts_hashmap_after_fetch_and_parse_len_counter =
        unfiltered_posts_hashmap_after_fetch_and_parse.len();
    let (unhandled_success_handled_success_are_there_items_yep_posts, some_error_posts) =
        rss_filter_fetched_and_parsed_posts(
            unfiltered_posts_hashmap_after_fetch_and_parse,
            provider_kind.clone(),
        );
    if unhandled_success_handled_success_are_there_items_yep_posts.is_empty() {
        print_colorful_message(
            Some(&provider_kind),
            PrintType::WarningHigh,
            file!().to_string(),
            line!().to_string(),
            "unhandled_success_handled_success_are_there_items_yep_posts is EMPTY!!!".to_string(),
        );
        (None, Some(some_error_posts))
    } else if unhandled_success_handled_success_are_there_items_yep_posts.len()
        != unfiltered_posts_hashmap_after_fetch_and_parse_len_counter
    {
        print_colorful_message(
                Some(&provider_kind),
                PrintType::PartialSuccess,
                file!().to_string(),
                line!().to_string(),
                format!(
                "(partially)succesfully_fetched_and_parsed_posts {} out of {} for {:#?}, allocated: {} byte/bytes",
                unhandled_success_handled_success_are_there_items_yep_posts.len(),
                unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
                provider_kind,
                mem::size_of_val(&unhandled_success_handled_success_are_there_items_yep_posts)
            ),
            );
        (
            Some(unhandled_success_handled_success_are_there_items_yep_posts),
            Some(some_error_posts),
        )
    } else {
        print_colorful_message(
            Some(&provider_kind),
            PrintType::Success,
            file!().to_string(),
            line!().to_string(),
            format!(
            "succesfully_fetched_and_parsed_posts {} out of {} for {:#?}, allocated: {} byte/bytes",
            unhandled_success_handled_success_are_there_items_yep_posts.len(),
            unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
            provider_kind,
            mem::size_of_val(&unhandled_success_handled_success_are_there_items_yep_posts)
        ),
        );
        (
            Some(unhandled_success_handled_success_are_there_items_yep_posts),
            None,
        )
    }
}
