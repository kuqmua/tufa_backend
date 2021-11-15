use crate::fetch::provider_log_into_json::provider_log_into_json;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::fetch::rss_write_error_logs_into_file_for_provider_wrapper_checker::rss_write_error_logs_into_file_for_provider_wrapper_checker;
use crate::providers::provider_kind_enum::ProviderKind;
use futures::future::join_all;
use std::time::Instant;

use crate::config_mods::config::CONFIG;

#[deny(clippy::indexing_slicing)] //, clippy::unwrap_used
#[tokio::main]
pub async fn rss_async_write_fetch_error_logs_into_files_wrapper(
    error_posts: Vec<(String, NoItemsError, ProviderKind)>,
) {
    let time = Instant::now();
    let mut vec_of_write_into_files_futures = Vec::with_capacity(error_posts.len());
    for (link, are_there_items, provider_kind) in error_posts {
        let json_object = provider_log_into_json(
            &link.clone(), //todo understand lifetimes to remove it
            &are_there_items,
            &provider_kind,
        );
        vec_of_write_into_files_futures
                    .push(rss_write_error_logs_into_file_for_provider_wrapper_checker(
                    json_object,
                    provider_kind,
                    &CONFIG
                        .params
                        .unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                    &CONFIG.params.warning_logs_directory_name,
                    link,
                ));
    }
    let _ = join_all(vec_of_write_into_files_futures).await; //todo: add state of success/unsuccess
    if CONFIG.params.enable_time_measurement_prints {
        println!(
            "write fetch error logs into files done in {} seconds {} miliseconds",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
        );
    }
}
