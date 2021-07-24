use crate::fetch::rss_part::rss_part;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use std::sync::{Arc, Mutex};

use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

use config_lib::get_project_information::get_config::get_config_information::CONFIG;

use config_lib::get_project_information::provider_kind_enum::ProviderKind;

pub fn providers_new_posts_check(
    provider_link: &str,
    provider_kind_handle_clone: ProviderKind,
    vec_of_provider_links: Vec<String>,
    option_provider_providers: Option<Vec<String>>,
    posts_handle: Arc<Mutex<Vec<CommonRssPostStruct>>>,
    error_posts_handle: Arc<
        Mutex<
            Vec<(
                String,
                UnhandledFetchStatusInfo,
                HandledFetchStatusInfo,
                AreThereItems,
                ProviderKind,
            )>,
        >,
    >,
) {
    let enum_success_unsuccess_option_posts = rss_part(
        provider_link,
        provider_kind_handle_clone,
        CONFIG.params.enable_error_prints,
        vec_of_provider_links,
        option_provider_providers,
    );
    if let Some(success_posts) = enum_success_unsuccess_option_posts.0 {
        let mut posts_handle_locked = posts_handle.lock().unwrap();
        for value in success_posts {
            posts_handle_locked.push(value);
        }
    }
    if let Some(unsuccess_posts) = enum_success_unsuccess_option_posts.1 {
        let mut error_posts_handle_locked = error_posts_handle.lock().unwrap();
        for unsuccess_post in unsuccess_posts {
            error_posts_handle_locked.push(unsuccess_post);
        }
    }
}
