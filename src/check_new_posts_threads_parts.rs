use std::collections::HashMap;

use futures::future::join_all;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::providers::provider_kind_impl::functions::rss_part::RssPartErrorEnum;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::providers::provider_kind_impl::functions::rss_part::rss_part;

#[deny(clippy::indexing_slicing)]
pub async fn check_new_posts_threads_parts(
    providers_link_parts: HashMap<ProviderKind, Vec<String>>,
) -> HashMap<ProviderKind, Result<Vec<CommonRssPostStruct>, RssPartErrorEnum>> {
    let tasks_vec = providers_link_parts
        .into_iter()
        .map(|(pk, link_parts)| async move {
            match rss_part(pk, pk.generate_provider_links(link_parts)).await {
                Ok(posts_vec) => (pk, Ok(posts_vec)),
                Err(e) => (pk, Err(*e)),
            }
        });
    let posts_and_errors_to_return = join_all(tasks_vec)
        .await
        .into_iter()
        .map(|(pk, s)| (pk, s))
        .collect();
    posts_and_errors_to_return
}
