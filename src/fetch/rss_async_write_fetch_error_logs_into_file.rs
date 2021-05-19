use crate::fetch::rss_handle_error_status_code::handle_error_status_code;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::fetch::rss_write_error_logs_into_file_for_provider::rss_write_error_logs_into_file_for_provider;
use crate::get_project_information::get_config::get_config_information::CONFIG;
use chrono::Local;
use serde_json::json;
use std::time::Instant;

#[allow(clippy::clippy::too_many_arguments)]
pub async fn rss_async_write_fetch_error_logs_into_file(
    value: (
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    ),
    unhandled_success_handled_success_are_there_items_initialized_posts_dir: &str,
    enable_time_measurement: bool,
    time: Instant,
    warning_logs_directory_name: &str,
) {
    let should_enable_prints: bool;
    let should_enable_error_prints: bool;
    match value.4 {
        ProviderKind::Arxiv => {
            should_enable_prints = CONFIG.enable_prints.enable_prints_arxiv;
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_arxiv;
        }
        ProviderKind::Biorxiv => {
            should_enable_prints = CONFIG.enable_prints.enable_prints_biorxiv;
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_biorxiv;
        }
        ProviderKind::Habr => {
            should_enable_prints = CONFIG.enable_prints.enable_prints_habr;
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_habr;
        }
        ProviderKind::Medrxiv => {
            should_enable_prints = CONFIG.enable_prints.enable_prints_medrxiv;
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_medrxiv;
        }
        ProviderKind::Reddit => {
            should_enable_prints = CONFIG.enable_prints.enable_prints_reddit;
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_reddit;
        }
        ProviderKind::Twitter => {
            should_enable_prints = CONFIG.enable_prints.enable_prints_twitter;
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_twitter;
        }
    }
    match value.1 {
        UnhandledFetchStatusInfo::Success => match value.2 {
            HandledFetchStatusInfo::Success => match value.3 {
                AreThereItems::Yep => (),
                AreThereItems::Initialized => {
                    let json_object = json!({
                        "link": &value.0,
                        "part_of": format!("{:?}", value.4),
                        "date": Local::now().to_string()
                    });
                    rss_write_error_logs_into_file_for_provider(
                        json_object,
                        &value.4,
                        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                        should_enable_prints,
                        should_enable_error_prints,
                        warning_logs_directory_name,
                        &value.0,
                    )
                }
                AreThereItems::NopeButThereIsTag(fetch_result_string) => {
                    //"</item>" tag
                    let json_object = json!({
                        "link": &value.0,
                        "fetch_result_string": fetch_result_string,
                        "part_of": format!("{:?}", value.4),
                        "date": Local::now().to_string()
                    });
                    rss_write_error_logs_into_file_for_provider(
                        json_object,
                        &value.4,
                        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                        should_enable_prints,
                        should_enable_error_prints,
                        warning_logs_directory_name,
                        &value.0,
                    )
                }
                AreThereItems::ConversionFromStrError(fetch_result_string, error) => {
                    let json_object = json!({
                        "link": &value.0,
                        "fetch_result_string": fetch_result_string,
                        "error": error,
                        "part_of": format!("{:?}", value.4),
                        "date": Local::now().to_string()
                    });
                    rss_write_error_logs_into_file_for_provider(
                        json_object,
                        &value.4,
                        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                        should_enable_prints,
                        should_enable_error_prints,
                        warning_logs_directory_name,
                        &value.0,
                    )
                }
                AreThereItems::NopeNoTag(fetch_result_string) => {
                    let json_object = json!({
                        "link": &value.0,
                        "page_content": fetch_result_string,
                        "part_of": format!("{:?}", value.4),
                        "date": Local::now().to_string()
                    });
                    rss_write_error_logs_into_file_for_provider(
                        json_object,
                        &value.4,
                        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                        should_enable_prints,
                        should_enable_error_prints,
                        warning_logs_directory_name,
                        &value.0,
                    )
                }
            },
            HandledFetchStatusInfo::Initialized => {
                let json_object = json!({
                    "link": &value.0,
                    "part_of": format!("{:?}", value.4),
                    "date": Local::now().to_string()
                });
                rss_write_error_logs_into_file_for_provider(
                    json_object,
                    &value.4,
                    unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                    should_enable_prints,
                    should_enable_error_prints,
                    warning_logs_directory_name,
                    &value.0,
                )
            }
            HandledFetchStatusInfo::ResToTextError(error) => {
                let json_object = json!({
                    "link": &value.0,
                    "error": error,
                    "part_of": format!("{:?}", value.4),
                    "date": Local::now().to_string()
                });
                rss_write_error_logs_into_file_for_provider(
                    json_object,
                    &value.4,
                    unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                    should_enable_prints,
                    should_enable_error_prints,
                    warning_logs_directory_name,
                    &value.0,
                )
            }
            HandledFetchStatusInfo::ResStatusError(status_code) => {
                let json_object = json!({
                    "link": &value.0,
                    "status_code": status_code.to_string(),
                    "part_of": format!("{:?}", value.4),
                    "date": Local::now().to_string()
                });
                handle_error_status_code(status_code, &value.0);
                rss_write_error_logs_into_file_for_provider(
                    json_object,
                    &value.4,
                    unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                    should_enable_prints,
                    should_enable_error_prints,
                    warning_logs_directory_name,
                    &value.0,
                )
            }
        },
        UnhandledFetchStatusInfo::Initialized => {
            let json_object = json!({
                "link": &value.0,
                "part_of": format!("{:?}", value.4),
                "date": Local::now().to_string()
            });
            rss_write_error_logs_into_file_for_provider(
                json_object,
                &value.4,
                unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                should_enable_prints,
                should_enable_error_prints,
                warning_logs_directory_name,
                &value.0,
            )
        }
        UnhandledFetchStatusInfo::Failure(box_dyn_error) => {
            let json_object = json!({
                "link": &value.0,
                "error": box_dyn_error,
                "part_of": format!("{:?}", value.4),
                "date": Local::now().to_string()
            });
            rss_write_error_logs_into_file_for_provider(
                json_object,
                &value.4,
                unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                should_enable_prints,
                should_enable_error_prints,
                warning_logs_directory_name,
                &value.0,
            )
        }
    }
    if enable_time_measurement {
        println!(
            "write fetch error logs into file done in {} seconds {} miliseconds for {:#?}",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
            value.4
        );
    }
}
