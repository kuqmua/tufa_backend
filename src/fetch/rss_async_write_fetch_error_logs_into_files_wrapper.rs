use crate::fetch::rss_async_write_fetch_error_logs_into_file::rss_async_write_fetch_error_logs_into_file;
use crate::fetch::rss_clean_logs_directory_wrapper::rss_clean_logs_directory_wrapper;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use futures::future::join_all;
use std::time::Instant;

use get_config_lib::CONFIG;

pub async fn rss_async_write_fetch_error_logs_into_files_wrapper(
    some_error_posts: Vec<(
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    )>,
) {
    let time = Instant::now();
    if CONFIG.params.enable_all_cleaning_warning_logs_directory {
        rss_clean_logs_directory_wrapper()
    }

    let mut vec_of_write_into_files_futures = Vec::with_capacity(some_error_posts.len());
    for some_error_post in some_error_posts {
        let enable_time_measurement: bool;
        match some_error_post.4 {
            ProviderKind::Arxiv => {
                enable_time_measurement = CONFIG.params.enable_common_time_measurement
                    && CONFIG.enable_time_measurement.enable_arxiv_time_measurement
            }
            ProviderKind::Biorxiv => {
                enable_time_measurement = CONFIG.params.enable_common_time_measurement
                    && CONFIG
                        .enable_time_measurement
                        .enable_biorxiv_time_measurement
            }
            ProviderKind::Github => {
                enable_time_measurement = CONFIG.params.enable_common_time_measurement
                    && CONFIG
                        .enable_time_measurement
                        .enable_github_time_measurement
            }
            ProviderKind::Habr => {
                enable_time_measurement = CONFIG.params.enable_common_time_measurement
                    && CONFIG.enable_time_measurement.enable_habr_time_measurement
            }
            ProviderKind::Medrxiv => {
                enable_time_measurement = CONFIG.params.enable_common_time_measurement
                    && CONFIG
                        .enable_time_measurement
                        .enable_medrxiv_time_measurement
            }
            ProviderKind::Reddit => {
                enable_time_measurement = CONFIG.params.enable_common_time_measurement
                    && CONFIG
                        .enable_time_measurement
                        .enable_reddit_time_measurement
            }
            ProviderKind::Twitter => {
                enable_time_measurement = CONFIG.params.enable_common_time_measurement
                    && CONFIG
                        .enable_time_measurement
                        .enable_twitter_time_measurement
            }
        }
        vec_of_write_into_files_futures.push(rss_async_write_fetch_error_logs_into_file(
            some_error_post,
            &CONFIG
                .params
                .unhandled_success_handled_success_are_there_items_initialized_posts_dir,
            enable_time_measurement,
            time,
            &CONFIG.params.warning_logs_directory_name,
        ));
    }
    let _ = join_all(vec_of_write_into_files_futures).await; //todo: add state of success/unsuccess
    if CONFIG.params.enable_all_time_measurement {
        println!(
            "write fetch error logs into files done in {} seconds {} miliseconds",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
        );
    }
}
