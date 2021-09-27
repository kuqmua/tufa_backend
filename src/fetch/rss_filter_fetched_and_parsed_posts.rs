use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

type FilterParsedSuccessErrorTuple = (
    Vec<CommonRssPostStruct>,
    Vec<(
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    )>,
);

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_filter_fetched_and_parsed_posts(
    unfiltered_posts_hashmap_after_fetch_and_parse: Vec<(
        CommonRssPostStruct,
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
    )>,
    provider_kind: ProviderKind,
) -> FilterParsedSuccessErrorTuple {
    let hashmap_length = unfiltered_posts_hashmap_after_fetch_and_parse.len();
    let mut unhandled_success_handled_success_are_there_items_yep_posts: Vec<CommonRssPostStruct> =
        Vec::new();
    let mut some_error_posts: Vec<(
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    )> = Vec::with_capacity(hashmap_length);
    for value in unfiltered_posts_hashmap_after_fetch_and_parse {
        match value.2 {
            UnhandledFetchStatusInfo::Success => match value.3 {
                HandledFetchStatusInfo::Success => match value.4 {
                    AreThereItems::Yep => {
                        unhandled_success_handled_success_are_there_items_yep_posts.push(value.0);
                    }
                    AreThereItems::Initialized => {
                        some_error_posts.push((
                            value.1,
                            value.2,
                            value.3,
                            AreThereItems::Initialized,
                            provider_kind,
                        ));
                    }
                    AreThereItems::NopeButThereIsTag(fetch_result_string) => {
                        //"</item>" tag
                        some_error_posts.push((
                            value.1,
                            value.2,
                            value.3,
                            AreThereItems::NopeButThereIsTag(fetch_result_string),
                            provider_kind,
                        ));
                    }
                    AreThereItems::ConversionFromStrError(fetch_result_string, error) => {
                        some_error_posts.push((
                            value.1,
                            value.2,
                            value.3,
                            AreThereItems::ConversionFromStrError(fetch_result_string, error),
                            provider_kind,
                        ));
                    }
                },
                HandledFetchStatusInfo::Initialized => {
                    some_error_posts.push((
                        value.1,
                        value.2,
                        HandledFetchStatusInfo::Initialized,
                        value.4,
                        provider_kind,
                    ));
                }
                HandledFetchStatusInfo::ResToTextError(error) => {
                    some_error_posts.push((
                        value.1,
                        value.2,
                        HandledFetchStatusInfo::ResToTextError(error),
                        value.4,
                        provider_kind,
                    ));
                }
                HandledFetchStatusInfo::ResStatusError(status_code) => {
                    some_error_posts.push((
                        value.1,
                        value.2,
                        HandledFetchStatusInfo::ResStatusError(status_code),
                        value.4,
                        provider_kind,
                    ));
                }
            },
            UnhandledFetchStatusInfo::Initialized => {
                some_error_posts.push((
                    value.1,
                    UnhandledFetchStatusInfo::Initialized,
                    value.3,
                    value.4,
                    provider_kind,
                ));
            }
            UnhandledFetchStatusInfo::Failure(box_dyn_error) => {
                some_error_posts.push((
                    value.1,
                    UnhandledFetchStatusInfo::Failure(box_dyn_error),
                    value.3,
                    value.4,
                    provider_kind,
                ));
            }
        }
    }
    (
        unhandled_success_handled_success_are_there_items_yep_posts,
        some_error_posts,
    )
}
