use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::fetch::rxiv_structures::RxivPostStruct;
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
    HashMap<
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
) {
    let hashmap_length = unfiltered_posts_hashmap_after_fetch_and_parse.len();
    let mut unhandled_success_handled_success_are_there_items_yep_posts: HashMap<
        String,
        RxivPostStruct,
    > = HashMap::new();
    let mut some_error_posts: HashMap<
        String,
        (
            RxivPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
            RxivKind,
        ),
    > = HashMap::with_capacity(hashmap_length);
    for (key, value) in unfiltered_posts_hashmap_after_fetch_and_parse {
        match value.2 {
            UnhandledFetchStatusInfo::Success => match value.3 {
                HandledFetchStatusInfo::Success => match value.4 {
                    AreThereItems::Yep => {
                        unhandled_success_handled_success_are_there_items_yep_posts
                            .insert(key, value.0);
                    }
                    AreThereItems::Initialized => {
                        some_error_posts.insert(key, value);
                    }
                    AreThereItems::NopeButThereIsTag(fetch_result_string) => {
                        //"</item>" tag
                        some_error_posts.insert(
                            key,
                            (
                                value.0,
                                value.1,
                                value.2,
                                value.3,
                                AreThereItems::NopeButThereIsTag(fetch_result_string),
                                value.5,
                            ),
                        );
                    }
                    AreThereItems::ConversionFromStrError(fetch_result_string, error) => {
                        some_error_posts.insert(
                            key,
                            (
                                value.0,
                                value.1,
                                value.2,
                                value.3,
                                AreThereItems::ConversionFromStrError(fetch_result_string, error),
                                value.5,
                            ),
                        );
                    }
                    AreThereItems::NopeNoTag(fetch_result_string) => {
                        some_error_posts.insert(
                            key,
                            (
                                value.0,
                                value.1,
                                value.2,
                                value.3,
                                AreThereItems::NopeNoTag(fetch_result_string),
                                value.5,
                            ),
                        );
                    }
                },
                HandledFetchStatusInfo::Initialized => {
                    some_error_posts.insert(
                        key,
                        (
                            value.0,
                            value.1,
                            value.2,
                            HandledFetchStatusInfo::Initialized,
                            value.4,
                            value.5,
                        ),
                    );
                }
                HandledFetchStatusInfo::ResToTextError(error) => {
                    some_error_posts.insert(
                        key,
                        (
                            value.0,
                            value.1,
                            value.2,
                            HandledFetchStatusInfo::ResToTextError(error),
                            value.4,
                            value.5,
                        ),
                    );
                }
                HandledFetchStatusInfo::ResStatusError(status_code) => {
                    // let should_refetch_it = handle_error_status_code(status_code);
                    some_error_posts.insert(
                        key,
                        (
                            value.0,
                            value.1,
                            value.2,
                            HandledFetchStatusInfo::ResStatusError(status_code),
                            value.4,
                            value.5,
                        ),
                    );
                }
            },
            UnhandledFetchStatusInfo::Initialized => {
                some_error_posts.insert(
                    key,
                    (
                        value.0,
                        value.1,
                        UnhandledFetchStatusInfo::Initialized,
                        value.3,
                        value.4,
                        value.5,
                    ),
                );
            }
            UnhandledFetchStatusInfo::Failure(box_dyn_error) => {
                some_error_posts.insert(
                    key,
                    (
                        value.0,
                        value.1,
                        UnhandledFetchStatusInfo::Failure(box_dyn_error),
                        value.3,
                        value.4,
                        value.5,
                    ),
                );
            }
        }
    }
    (
        unhandled_success_handled_success_are_there_items_yep_posts,
        some_error_posts,
    )
}
