extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::fetch::rss_filter_fetched_and_parsed_posts::rss_filter_fetched_and_parsed_posts;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::overriding::prints::print_partial_success_cyan;
use crate::overriding::prints::print_success_green;
use crate::overriding::prints::print_warning_orange;
use std::collections::HashMap;
use std::mem;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

#[allow(clippy::clippy::too_many_arguments)]
pub fn rss_handle_unfiltered_posts(
    unfiltered_posts_hashmap_after_fetch_and_parse: Vec<(
        String,
        (
            CommonRssPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
        ),
    )>,
    provider_kind: &'static ProviderKind,
    enable_prints: bool,
    enable_warning_prints: bool,
) -> (
    Option<HashMap<String, CommonRssPostStruct>>,
    Option<
        Vec<(
            CommonRssPostStruct,
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
            unfiltered_posts_hashmap_after_fetch_and_parse.to_vec(),
            provider_kind,
        );
    if unhandled_success_handled_success_are_there_items_yep_posts.is_empty() {
        if enable_warning_prints {
            print_warning_orange(
                file!().to_string(),
                line!().to_string(),
                "unhandled_success_handled_success_are_there_items_yep_posts is EMPTY!!!"
                    .to_string(),
            );
        }
        (None, Some(some_error_posts))
    } else if unhandled_success_handled_success_are_there_items_yep_posts.len()
        != unfiltered_posts_hashmap_after_fetch_and_parse_len_counter
    {
        if enable_prints {
            let message = format!(
                                        "(partially)succesfully_fetched_and_parsed_posts {} out of {} for {:#?}, allocated: {} byte/bytes",
                                        unhandled_success_handled_success_are_there_items_yep_posts.len(),
                                        unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
                                        provider_kind,
                                        mem::size_of_val(&unhandled_success_handled_success_are_there_items_yep_posts)
                                    );
            print_partial_success_cyan(file!().to_string(), line!().to_string(), message);
        }
        (
            Some(unhandled_success_handled_success_are_there_items_yep_posts),
            Some(some_error_posts),
        )
    } else {
        let message = format!(
            "succesfully_fetched_and_parsed_posts {} out of {} for {:#?}, allocated: {} byte/bytes",
            unhandled_success_handled_success_are_there_items_yep_posts.len(),
            unfiltered_posts_hashmap_after_fetch_and_parse_len_counter,
            provider_kind,
            mem::size_of_val(&unhandled_success_handled_success_are_there_items_yep_posts)
        );
        if enable_prints {
            print_success_green(file!().to_string(), line!().to_string(), message);
        }
        (
            Some(unhandled_success_handled_success_are_there_items_yep_posts),
            None,
        )
    }
}
