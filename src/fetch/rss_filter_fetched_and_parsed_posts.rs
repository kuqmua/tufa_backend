use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::fetch::rss_structures::RxivPostStruct;
use std::collections::HashMap;

#[allow(clippy::clippy::too_many_arguments, clippy::clippy::type_complexity)]
pub fn rss_filter_fetched_and_parsed_posts(
    unfiltered_posts_hashmap_after_fetch_and_parse: Vec<(
        String,
        (
            RxivPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
        ),
    )>,
    provider_kind: &ProviderKind,
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
            ProviderKind,
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
            ProviderKind,
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
                        some_error_posts.insert(
                            key,
                            (
                                value.0,
                                value.1,
                                value.2,
                                value.3,
                                AreThereItems::Initialized,
                                provider_kind.clone(),
                            ),
                        );
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
                                provider_kind.clone(),
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
                                provider_kind.clone(),
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
                                provider_kind.clone(),
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
                            provider_kind.clone(),
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
                            provider_kind.clone(),
                        ),
                    );
                }
                HandledFetchStatusInfo::ResStatusError(status_code) => {
                    some_error_posts.insert(
                        key,
                        (
                            value.0,
                            value.1,
                            value.2,
                            HandledFetchStatusInfo::ResStatusError(status_code),
                            value.4,
                            provider_kind.clone(),
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
                        provider_kind.clone(),
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
                        provider_kind.clone(),
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
