use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::fetch::rss_part::{rss_part, RssPartError};

use std::sync::{Arc, Mutex};

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
// use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[derive(Debug, Clone)]
pub enum NewPostsCheckError {
    PostErrorVariant(PostErrorVariant),
    RssPartError(RssPartError),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn providers_new_posts_check(
    provider_kind: ProviderKind,
    vec_of_provider_links: Vec<String>,
    posts_handle: Arc<Mutex<Vec<CommonRssPostStruct>>>,
    error_posts_handle: Arc<Mutex<Vec<NewPostsCheckError>>>,
) {
    let result = rss_part(provider_kind, vec_of_provider_links);
    match result.1 {
        Ok((vec_common_rss_post_structs, vec_post_error_variants)) => {
            //maybe do it in parrallel? success and error posts
            //todo: try to lock few times
            if !vec_common_rss_post_structs.is_empty() {
                //must check on empty coz lock it for nothing is bad
                match posts_handle.lock() {
                    Ok(mut posts_handle_locked) => {
                        for post in vec_common_rss_post_structs {
                            posts_handle_locked.push(post);
                        }
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
            if !vec_post_error_variants.is_empty() {
                //must check on empty coz lock it for nothing is bad
                match error_posts_handle.lock() {
                    Ok(mut error_posts_handle_locked) => {
                        for post_error_variant in vec_post_error_variants {
                            error_posts_handle_locked
                                .push(NewPostsCheckError::PostErrorVariant(post_error_variant))
                        }
                    }
                    Err(e) => {
                        print_colorful_message(
                            None,
                            PrintType::Error,
                            file!().to_string(),
                            line!().to_string(),
                            format!(
                                "error_posts_handle.lock() (unsuccess_posts_posts) error: {:#?}",
                                e
                            ),
                        );
                    }
                }
            }
        }
        Err(e) => {
            //todo
        }
    }
}
