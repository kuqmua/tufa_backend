use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::providers::provider_kind_enum::ProviderKind;

type FilterParsedSuccessErrorTuple = (
    Vec<CommonRssPostStruct>,
    Vec<(String, AreThereItems, ProviderKind)>,
);

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_filter_fetched_and_parsed_posts(
    unfiltered_posts_hashmap_after_fetch_and_parse: Vec<
        Result<(CommonRssPostStruct, String, AreThereItems), String>,
    >,
    provider_kind: ProviderKind,
) -> FilterParsedSuccessErrorTuple {
    let hashmap_length = unfiltered_posts_hashmap_after_fetch_and_parse.len();
    let mut unhandled_success_handled_success_are_there_items_yep_posts: Vec<CommonRssPostStruct> =
        Vec::new();
    let mut some_error_posts: Vec<(String, AreThereItems, ProviderKind)> =
        Vec::with_capacity(hashmap_length);
    for result in unfiltered_posts_hashmap_after_fetch_and_parse {
        match result {
            Ok((post, link, are_there_items)) => {
                match are_there_items {
                    AreThereItems::Yep => {
                        unhandled_success_handled_success_are_there_items_yep_posts.push(post);
                    }
                    AreThereItems::NopeButThereIsTag(fetch_result_string) => {
                        //"</item>" tag
                        some_error_posts.push((
                            link,
                            AreThereItems::NopeButThereIsTag(fetch_result_string),
                            provider_kind,
                        ));
                    }
                    AreThereItems::ConversionFromStrError(fetch_result_string, error) => {
                        some_error_posts.push((
                            link,
                            AreThereItems::ConversionFromStrError(fetch_result_string, error),
                            provider_kind,
                        ));
                    }
                    AreThereItems::NopeNoTag(tag) => {
                        some_error_posts.push((link, AreThereItems::NopeNoTag(tag), provider_kind));
                    }
                }
            }
            Err(string_error) => {
                todo!()
            }
        }
    }
    (
        unhandled_success_handled_success_are_there_items_yep_posts,
        some_error_posts,
    )
}
