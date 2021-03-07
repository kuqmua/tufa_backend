use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::fetch::rxiv_structures::RxivPostStruct;
use reqwest::StatusCode;
use std::collections::HashMap;

#[allow(clippy::clippy::too_many_arguments, clippy::clippy::type_complexity)]
pub fn rxiv_filter_fetched_and_parsed_posts(
    unfiltered_posts_hashmap_after_fetch_and_parse: HashMap<
        String,
        (
            RxivPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
            RxivKind,
        ),
    >,
) -> (
    HashMap<String, RxivPostStruct>,
    std::collections::HashMap<std::string::String, std::string::String>,
    std::collections::HashMap<std::string::String, (std::string::String, std::string::String)>,
    std::collections::HashMap<
        std::string::String,
        (
            std::string::String,
            std::string::String,
            std::string::String,
        ),
    >,
    std::collections::HashMap<std::string::String, (std::string::String, std::string::String)>,
    std::collections::HashMap<std::string::String, std::string::String>,
    std::collections::HashMap<std::string::String, (std::string::String, std::string::String)>,
    std::collections::HashMap<std::string::String, (std::string::String, StatusCode)>,
    std::collections::HashMap<std::string::String, std::string::String>,
    std::collections::HashMap<std::string::String, (std::string::String, std::string::String)>,
) {
    let mut unhandled_success_handled_success_are_there_items_yep_posts: HashMap<
        String,
        RxivPostStruct,
    > = HashMap::new();
    let mut unhandled_success_handled_success_are_there_items_initialized_posts: HashMap<
        String,
        String,
    > = HashMap::new();
    let mut unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts: HashMap<
        String,
        (String, String),
    > = HashMap::new(); //"</item>" tag
    let mut unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts: HashMap<
            String,
            (String, String, String),
        > = HashMap::new();
    let mut unhandled_success_handled_success_are_there_items_nope_no_tag_posts: HashMap<
        String,
        (String, String),
    > = HashMap::new();
    /////
    let mut unhandled_success_handled_initialized_posts: HashMap<String, String> = HashMap::new();
    let mut unhandled_success_handled_res_to_text_error_posts: HashMap<String, (String, String)> =
        HashMap::new();
    let mut unhandled_success_handled_res_status_error_posts: HashMap<
        String,
        (String, StatusCode),
    > = HashMap::new();
    //////
    let mut unhandled_initialized_posts: HashMap<String, String> = HashMap::new();
    let mut unhandled_failure_posts: HashMap<String, (String, String)> = HashMap::new();
    for (key, value) in unfiltered_posts_hashmap_after_fetch_and_parse {
        match value.2 {
            UnhandledFetchStatusInfo::Success => match value.3 {
                HandledFetchStatusInfo::Success => match value.4 {
                    AreThereItems::Yep => {
                        unhandled_success_handled_success_are_there_items_yep_posts
                            .insert(key, value.0);
                    }
                    AreThereItems::Initialized => {
                        unhandled_success_handled_success_are_there_items_initialized_posts
                            .insert(key, value.1);
                    }
                    AreThereItems::NopeButThereIsTag(fetch_result_string) => {
                        //"</item>" tag
                        unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts
                                .insert(key, (value.1, fetch_result_string));
                    }
                    AreThereItems::ConversionFromStrError(fetch_result_string, error) => {
                        unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts
                                .insert(key, (value.1, fetch_result_string, error));
                    }
                    AreThereItems::NopeNoTag(fetch_result_string) => {
                        unhandled_success_handled_success_are_there_items_nope_no_tag_posts
                            .insert(key, (value.1, fetch_result_string));
                    }
                },
                HandledFetchStatusInfo::Initialized => {
                    unhandled_success_handled_initialized_posts.insert(key, value.1);
                }
                HandledFetchStatusInfo::ResToTextError(error) => {
                    unhandled_success_handled_res_to_text_error_posts.insert(key, (value.1, error));
                }
                HandledFetchStatusInfo::ResStatusError(status_code) => {
                    // let should_refetch_it = handle_error_status_code(status_code);
                    unhandled_success_handled_res_status_error_posts
                        .insert(key, (value.1, status_code));
                }
            },
            UnhandledFetchStatusInfo::Initialized => {
                unhandled_initialized_posts.insert(key, value.1);
            }
            UnhandledFetchStatusInfo::Failure(box_dyn_error) => {
                unhandled_failure_posts.insert(key, (value.1, box_dyn_error));
            }
        }
    }
    (
        unhandled_success_handled_success_are_there_items_yep_posts,
        unhandled_success_handled_success_are_there_items_initialized_posts,
        unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts,
        unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts,
        unhandled_success_handled_success_are_there_items_nope_no_tag_posts,
        unhandled_success_handled_initialized_posts,
        unhandled_success_handled_res_to_text_error_posts,
        unhandled_success_handled_res_status_error_posts,
        unhandled_initialized_posts,
        unhandled_failure_posts,
    )
}
