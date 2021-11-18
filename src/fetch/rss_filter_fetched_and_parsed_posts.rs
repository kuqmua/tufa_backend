use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::providers::provider_kind_enum::ProviderKind;

#[derive(Debug, Clone)]
pub enum PostErrorVariant {
    //todo: think about this naming
    NoItems {
        link: String,
        no_items_error: NoItemsError,
        provider_kind: ProviderKind,
    },
    RssFetchAndParseProviderDataError {
        link: String,
        provider_kind: ProviderKind,
        error: String, //it must be different type but dont know how to clone error to different thread
    }, //rewrite this error coz it must not be string. dont know to to clone error between threads
}

type FilterParsedSuccessErrorTuple = (Vec<CommonRssPostStruct>, Vec<PostErrorVariant>);

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_filter_fetched_and_parsed_posts(
    unfiltered_posts_hashmap_after_fetch_and_parse: Vec<
        Result<Result<CommonRssPostStruct, (NoItemsError, String)>, (String, ProviderKind, String)>,
    >, //it must be enum
    provider_kind: ProviderKind,
) -> FilterParsedSuccessErrorTuple {
    let mut unhandled_success_handled_success_are_there_items_yep_posts: Vec<CommonRssPostStruct> =
        Vec::with_capacity(unfiltered_posts_hashmap_after_fetch_and_parse.len());
    let mut some_error_posts: Vec<PostErrorVariant> =
        Vec::with_capacity(unfiltered_posts_hashmap_after_fetch_and_parse.len());
    for result in unfiltered_posts_hashmap_after_fetch_and_parse {
        match result {
            Ok(second_result) => match second_result {
                Ok(post) => {
                    unhandled_success_handled_success_are_there_items_yep_posts.push(post);
                }
                Err((err, link)) => {
                    match err {
                        NoItemsError::ThereIsTag(fetch_result_string) => {
                            //"</item>" tag
                            some_error_posts.push(PostErrorVariant::NoItems {
                                link,
                                no_items_error: NoItemsError::ThereIsTag(fetch_result_string),
                                provider_kind,
                            });
                        }
                        NoItemsError::ConversionFromStrError(fetch_result_string, error) => {
                            some_error_posts.push(PostErrorVariant::NoItems {
                                link,
                                no_items_error: NoItemsError::ConversionFromStrError(
                                    fetch_result_string,
                                    error,
                                ),
                                provider_kind,
                            });
                        }
                        NoItemsError::NoTag(tag) => {
                            some_error_posts.push(PostErrorVariant::NoItems {
                                link,
                                no_items_error: NoItemsError::NoTag(tag),
                                provider_kind,
                            });
                        }
                    }
                }
            },
            Err((link, provider_kind, string_error)) => {
                some_error_posts.push(PostErrorVariant::RssFetchAndParseProviderDataError {
                    link,
                    provider_kind,
                    error: string_error,
                }) //rewrite this error coz it must not be string. dont know to to clone error between threads
            }
        }
    }
    (
        unhandled_success_handled_success_are_there_items_yep_posts,
        some_error_posts,
    )
}
