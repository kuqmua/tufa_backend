use crate::fetch;
use crate::fetch::provider_kind_enum::ProviderKind;
use crate::fetch::rxiv::metainfo_fetch_structures::AreThereItems;
use crate::fetch::rxiv::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rxiv::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::twitter::twitter_async_write_fetch_error_logs_into_file::twitter_async_write_fetch_error_logs_into_file;
use crate::fetch::twitter::twitter_structures::TwitterPostStruct;
use futures::future::join_all;
use std::collections::HashMap;
use std::time::Instant;

pub async fn twitter_async_write_fetch_error_logs_into_files_wrapper(
    provider_kind: &'static fetch::provider_kind_enum::ProviderKind,
    enable_prints: bool,
    // enable_warning_prints: bool,
    enable_error_prints: bool,
    enable_time_measurement: bool,
    some_error_posts: HashMap<
        String,
        (
            TwitterPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
            ProviderKind,
        ),
    >,
) {
    let time = Instant::now();
    let unhandled_success_handled_success_are_there_items_initialized_posts_dir =
        "unhandled_success_handled_success_are_there_items_initialized_posts";
    let mut vec_of_write_into_files_futures = Vec::with_capacity(some_error_posts.len());
    for (key, value) in some_error_posts {
        vec_of_write_into_files_futures.push(twitter_async_write_fetch_error_logs_into_file(
            key,
            value,
            unhandled_success_handled_success_are_there_items_initialized_posts_dir,
            provider_kind,
            enable_prints,
            enable_error_prints,
            enable_time_measurement,
            time,
        ));
    }
    let _ = join_all(vec_of_write_into_files_futures).await; //todo: add state of success/unsuccess
    if enable_time_measurement {
        println!(
            "write fetch error logs into files done in {} seconds {} miliseconds for {:#?}",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
            provider_kind
        );
    }
}
