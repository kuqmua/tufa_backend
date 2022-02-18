use std::collections::HashMap;

use std::sync::Arc;

use tokio::sync::Mutex;

use futures::future::join_all;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::providers::provider_kind_impl::functions::rss_part::RssPartErrorEnum;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::providers_new_posts_check::providers_new_posts_check;

use crate::helpers::get_git_commit_string::get_git_commit_string;
use crate::helpers::resource::Resource;

use crate::traits::git_info_trait::GitInfo;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[derive(Debug, GitInfoDerive)]
pub enum ResourceError {
    NoLinkParts(Resource),
    Other,
}

#[deny(clippy::indexing_slicing)]
pub async fn check_new_posts_threads_parts(
    providers_link_parts: HashMap<ProviderKind, Vec<String>>,
) -> Result<HashMap<ProviderKind, Result<Vec<CommonRssPostStruct>, RssPartErrorEnum>>, ResourceError>
{
    
    let posts_and_errors_arc_mutex = Arc::new(Mutex::new(HashMap::with_capacity(//maybe it needs only Muxex? without Arc?
        providers_link_parts.len(),
    )));
    //check if provider_names are unique
    let tasks_vec = providers_link_parts.into_iter().map(|(pk, link_parts)|{
        let posts_and_errors_handle_arc = Arc::clone(&posts_and_errors_arc_mutex);
        async move {
             providers_new_posts_check(
                pk,
                pk.generate_provider_links(link_parts),
                posts_and_errors_handle_arc
            )
            .await;
        }
    });
    let _ = join_all(tasks_vec).await;
    let posts_and_errors_to_return = posts_and_errors_arc_mutex.lock().await.drain().collect();
    Ok(posts_and_errors_to_return)
}
