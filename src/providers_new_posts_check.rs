use crate::fetch::rss_part::rss_part;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use std::sync::{Arc, Mutex};

use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

use crate::get_project_information::get_config::get_lazy_config_information::CONFIG;

use crate::get_project_information::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

type ArcMutexErrorPostsHandle = Arc<
    Mutex<
        Vec<(
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
            ProviderKind,
        )>,
    >,
>;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn providers_new_posts_check(
    provider_link: &str,
    provider_kind_handle_clone: ProviderKind,
    vec_of_provider_links: Vec<String>,
    option_provider_providers: Option<Vec<String>>,
    posts_handle: Arc<Mutex<Vec<CommonRssPostStruct>>>,
    error_posts_handle: ArcMutexErrorPostsHandle,
) {
    let enum_success_unsuccess_option_posts = rss_part(
        provider_link,
        provider_kind_handle_clone,
        CONFIG.params.enable_error_prints,
        vec_of_provider_links,
        option_provider_providers,
    );
    //maybe do it in parrallel? success and error posts
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
