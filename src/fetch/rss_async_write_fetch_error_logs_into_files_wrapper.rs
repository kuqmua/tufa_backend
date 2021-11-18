use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::fetch::rss_write_error_logs_into_file_for_provider_wrapper_checker::rss_write_error_logs_into_file_for_provider_wrapper_checker;
use crate::providers::provider_kind_enum::ProviderKind;
use chrono::Local;
use futures::future::join_all;
use serde_json::json;
use std::time::Instant;

use crate::config_mods::config::CONFIG;

#[deny(clippy::indexing_slicing)] //, clippy::unwrap_used
#[tokio::main]
pub async fn rss_async_write_fetch_error_logs_into_files_wrapper(
    error_posts: Vec<PostErrorVariant>,
) {
    let time = Instant::now();
    let mut vec_of_write_into_files_futures = Vec::with_capacity(error_posts.len());
    for post_error_variant in error_posts {
        match post_error_variant {
            PostErrorVariant::NoItems {
                link,
                no_items_error,
                provider_kind,
            } => {
                let json_object = NoItemsError::into_json_with_link_and_provider_kind(
                    &link.clone(), //todo understand lifetimes to remove it
                    &no_items_error,
                    &provider_kind,
                );
                let replaced_link = link.replace("/", "-").replace(":", "-").replace(".", "-");
                let path_to_provider_log_file =
                    format!(
                    "logs/{}/{:?}/{}",
                    &CONFIG.params.warning_logs_directory_name, provider_kind, &CONFIG
                    .params
                    .unhandled_success_handled_success_are_there_items_initialized_posts_dir
                );
                let path_to_file = format!(
                    "logs/{}/{:?}/{}/{:?}-{}.json",
                    &CONFIG.params.warning_logs_directory_name,
                    ProviderKind::get_string_name(provider_kind),
                    &CONFIG
                        .params
                        .unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                    ProviderKind::get_string_name(provider_kind),
                    replaced_link
                ); //add save function what convert string into save path
                vec_of_write_into_files_futures.push(
                    rss_write_error_logs_into_file_for_provider_wrapper_checker(
                        json_object,
                        provider_kind,
                        path_to_provider_log_file,
                        path_to_file,
                    ),
                );
            }
            PostErrorVariant::RssFetchAndParseProviderDataError{
                link, 
                provider_kind, 
                error: string_error
            } => {
                let replaced_link = link.replace("/", "-").replace(":", "-").replace(".", "-");
                let path_to_provider_log_file =
                    format!(
                    "logs/{}/{:?}/{}",
                    &CONFIG.params.warning_logs_directory_name, provider_kind, &CONFIG
                    .params
                    .unhandled_success_handled_success_are_there_items_initialized_posts_dir
                );
                let path_to_file = format!(
                    "logs/{}/{:?}/{}/{:?}-{}.json",
                    &CONFIG.params.warning_logs_directory_name,
                    ProviderKind::get_string_name(provider_kind),
                    &CONFIG
                        .params
                        .unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                    ProviderKind::get_string_name(provider_kind),
                    replaced_link
                ); //add save function what convert string into save path
                let json_object = json!({
                    "link": link,
                    "stringified_error": string_error,
                    "part_of": ProviderKind::get_string_name(provider_kind),
                    "date": Local::now().to_string()
                });
                vec_of_write_into_files_futures.push(
                    rss_write_error_logs_into_file_for_provider_wrapper_checker(
                        json_object,
                        provider_kind,
                        path_to_provider_log_file,
                        path_to_file,
                    ),
                );
            }
        }
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
