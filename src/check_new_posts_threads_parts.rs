use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use futures::future::join_all;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;
use crate::fetch::rss_part::RssPartError;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::providers_new_posts_check::providers_new_posts_check;

use crate::helpers::resource::Resource;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[derive(Debug)]
pub enum ResourceError {
    NoLinkParts(Resource),
    Other,
}

#[deny(clippy::indexing_slicing)]
pub async fn check_new_posts_threads_parts(
    providers_link_parts: HashMap<ProviderKind, Vec<String>>,
) -> Result<
    Vec<(
        ProviderKind,
        Result<(Vec<CommonRssPostStruct>, Vec<PostErrorVariant>), RssPartError>,
    )>,
    ResourceError,
> {
    let mut tasks_vec = Vec::with_capacity(providers_link_parts.len());
    let posts_and_errors = Vec::with_capacity(providers_link_parts.len());
    let posts_and_errors_arc_mutex = Arc::new(Mutex::new(posts_and_errors));
    //check if provider_names are unique
    for (provider_kind, link_parts) in providers_link_parts {
        if !link_parts.is_empty() {
            let posts_and_errors_handle_arc = Arc::clone(&posts_and_errors_arc_mutex);
            let vec_of_provider_links = provider_kind.generate_provider_links(link_parts);
            tasks_vec.push(tokio::task::spawn(async move {
                providers_new_posts_check(
                    provider_kind,
                    vec_of_provider_links,
                    posts_and_errors_handle_arc,
                );
            }));
        }
    }
    let _ = join_all(tasks_vec).await;
    let posts_and_errors_to_return = posts_and_errors_arc_mutex
        .lock()
        .unwrap()
        .drain(..)
        .collect();
    Ok(posts_and_errors_to_return)
}
