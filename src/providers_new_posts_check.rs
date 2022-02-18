use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;

use crate::providers::provider_kind_impl::functions::rss_part::rss_part;
use crate::providers::provider_kind_impl::functions::rss_part::RssPartErrorEnum;

use crate::providers::provider_kind_enum::ProviderKind;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn providers_new_posts_check(
    pk: ProviderKind,
    vec_of_provider_links: Vec<String>,
    posts_and_errors_arc_mutex: Arc<
        Mutex<HashMap<ProviderKind, Result<Vec<CommonRssPostStruct>, RssPartErrorEnum>>>,
    >,
) {
    match rss_part(pk, vec_of_provider_links).await {
        Ok(posts_vec) => {
            //maybe do it in parrallel? success and error posts
            if !posts_vec.is_empty() {
                //must check on empty coz lock it for nothing is bad
                let mut posts_handle_locked = posts_and_errors_arc_mutex.lock().await;
                posts_handle_locked.insert(pk, Ok(posts_vec));
            }
        }
        Err(e) => {
            let mut posts_handle_locked = posts_and_errors_arc_mutex.lock().await;
            posts_handle_locked.insert(pk, Err(*e));
        }
    }
}
