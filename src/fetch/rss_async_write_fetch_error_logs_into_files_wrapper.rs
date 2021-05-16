use crate::fetch::rss_async_write_fetch_error_logs_into_file::rss_async_write_fetch_error_logs_into_file;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use futures::future::join_all;
use std::time::Instant;

pub async fn rss_async_write_fetch_error_logs_into_files_wrapper(
    enable_time_measurement: bool,
    some_error_posts: Vec<(
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    )>,
    warning_logs_directory_name: &str,
) {
    let time = Instant::now();
    let unhandled_success_handled_success_are_there_items_initialized_posts_dir =
        "unhandled_success_handled_success_are_there_items_initialized_posts";
    let mut vec_of_write_into_files_futures = Vec::with_capacity(some_error_posts.len());
    for some_error_post in some_error_posts {
        vec_of_write_into_files_futures.push(rss_async_write_fetch_error_logs_into_file(
            some_error_post,
            unhandled_success_handled_success_are_there_items_initialized_posts_dir,
            enable_time_measurement,
            time,
            warning_logs_directory_name,
        ));
    }
    let _ = join_all(vec_of_write_into_files_futures).await; //todo: add state of success/unsuccess
    if enable_time_measurement {
        println!(
            "write fetch error logs into files done in {} seconds {} miliseconds",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
        );
    }
}
