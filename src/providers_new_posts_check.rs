use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;
use crate::fetch::rss_part::{rss_part, RssPartError};

use std::sync::{Arc, Mutex};

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
// use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn providers_new_posts_check(
    provider_kind: ProviderKind,
    vec_of_provider_links: Vec<String>,
    posts_and_errors_arc_mutex: Arc<
        Mutex<
            Vec<(
                ProviderKind,
                Result<(Vec<CommonRssPostStruct>, Vec<PostErrorVariant>), RssPartError>,
            )>,
        >,
    >,
) {
    let result = rss_part(provider_kind, vec_of_provider_links).await;
    match result {
        Ok((vec_common_rss_post_structs, vec_post_error_variants)) => {
            //maybe do it in parrallel? success and error posts
            //todo: try to lock few times
            if !vec_common_rss_post_structs.is_empty() && !vec_post_error_variants.is_empty() {
                //must check on empty coz lock it for nothing is bad
                match posts_and_errors_arc_mutex.lock() {
                    Ok(mut posts_handle_locked) => {
                        posts_handle_locked.push((
                            provider_kind,
                            Ok((vec_common_rss_post_structs, vec_post_error_variants)),
                        ));
                    }
                    Err(e) => {
                        print_colorful_message(
                            None,
                            PrintType::Error,
                            file!().to_string(),
                            line!().to_string(),
                            format!("posts_handle.lock() (success_posts) error: {:#?}", e),
                        );
                    }
                }
            }
        }
        Err(e) => match posts_and_errors_arc_mutex.lock() {
            Ok(mut posts_handle_locked) => {
                posts_handle_locked.push((provider_kind, Err(e)));
            }
            Err(e) => {
                print_colorful_message(
                    None,
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("posts_handle.lock() error: {:#?}", e),
                );
            }
        },
    }
}
