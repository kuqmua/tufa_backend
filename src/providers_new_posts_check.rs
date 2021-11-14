use crate::fetch::rss_part::rss_part;

use std::sync::{Arc, Mutex};

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

type ArcMutexErrorPostsHandle = Arc<Mutex<Vec<(String, AreThereItems, ProviderKind)>>>;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn providers_new_posts_check(
    provider_kind: ProviderKind,
    vec_of_provider_links: Vec<String>,
    posts_handle: Arc<Mutex<Vec<CommonRssPostStruct>>>,
    error_posts_handle: ArcMutexErrorPostsHandle,
) {
    let enum_success_unsuccess_option_posts = rss_part(provider_kind, vec_of_provider_links);
    //maybe do it in parrallel? success and error posts
    //todo: try to lock few times
    if let Some(success_posts) = enum_success_unsuccess_option_posts.0 {
        match posts_handle.lock() {
            Ok(mut posts_handle_locked) => {
                for success_post in success_posts {
                    posts_handle_locked.push(success_post);
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
    if let Some(unsuccess_posts) = enum_success_unsuccess_option_posts.1 {
        match error_posts_handle.lock() {
            Ok(mut error_posts_handle_locked) => {
                for unsuccess_post in unsuccess_posts {
                    error_posts_handle_locked.push(unsuccess_post);
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
