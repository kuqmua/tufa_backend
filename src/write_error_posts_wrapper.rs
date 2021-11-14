use crate::fetch::rss_async_write_fetch_error_logs_into_files_wrapper::rss_async_write_fetch_error_logs_into_files_wrapper;
use crate::logs_logic::async_write_fetch_error_logs_into_mongo_wrapper::async_write_fetch_error_logs_into_mongo_wrapper;
use crate::logs_logic::async_write_fetch_error_logs_into_mongo_wrapper::WriteLogsResult;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;

#[deny(clippy::indexing_slicing)]
#[tokio::main]
pub async fn write_error_posts_wrapper(error_posts: Vec<(String, AreThereItems, ProviderKind)>) {
    //todo add flag in config or if its already exists put it here
    if CONFIG.params.enable_write_error_logs_in_local_folder {
        let cleaning_hashmap_result = ProviderKind::clean_providers_logs_directory();
        //todo add enable_writing logs if not clean or not enabled cleaning
        if cleaning_hashmap_result.is_empty() {
            rss_async_write_fetch_error_logs_into_files_wrapper(error_posts.clone());
        } else {
            for (provider_kind, error) in cleaning_hashmap_result {
                print_colorful_message(
                                    Some(&provider_kind),
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("ProviderKind::clean_providers_logs_directory() failed for {:#?} (todo2) error: {:#?}", provider_kind, error),
                                );
            }
        }
    }
    if CONFIG.params.enable_write_error_logs_in_mongo {
        let result = async_write_fetch_error_logs_into_mongo_wrapper(error_posts).await;
        match result {
            WriteLogsResult::Success => {
                print_colorful_message(
                    None,
                    PrintType::Success,
                    file!().to_string(),
                    line!().to_string(),
                    format!(
                        "async_write_fetch_error_logs_into_mongo_wrapper result {:#?}",
                        result
                    ),
                );
            }
            WriteLogsResult::PartialSuccess => {
                print_colorful_message(
                    None,
                    PrintType::PartialSuccess,
                    file!().to_string(),
                    line!().to_string(),
                    format!(
                        "async_write_fetch_error_logs_into_mongo_wrapper result {:#?}",
                        result
                    ),
                );
            }
            WriteLogsResult::Failure => {
                print_colorful_message(
                    None,
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!(
                        "async_write_fetch_error_logs_into_mongo_wrapper result {:#?}",
                        result
                    ),
                );
            }
        }
    }
}
