use crate::fetch::rss_handle_error_status_code::handle_error_status_code;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::fetch::rss_write_error_logs_into_file::rss_write_error_logs_into_file;
use crate::get_project_information::get_config::get_config_information::CONFIG;
use chrono::Local;
use serde_json::json;
use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::overriding::prints::print_error_red;

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
    match value.4 {
        ProviderKind::Arxiv => should_enable_prints = CONFIG.enable_prints.enable_prints_arxiv,
        ProviderKind::Biorxiv => should_enable_prints = CONFIG.enable_prints.enable_prints_biorxiv,
        ProviderKind::Habr => should_enable_prints = CONFIG.enable_prints.enable_prints_habr,
        ProviderKind::Medrxiv => should_enable_prints = CONFIG.enable_prints.enable_prints_medrxiv,
        ProviderKind::Reddit => should_enable_prints = CONFIG.enable_prints.enable_prints_reddit,
        ProviderKind::Twitter => should_enable_prints = CONFIG.enable_prints.enable_prints_twitter,
    }
    let should_enable_error_prints: bool;
    match value.4 {
        ProviderKind::Arxiv => {
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_arxiv
        }
        ProviderKind::Biorxiv => {
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_biorxiv
        }
        ProviderKind::Habr => {
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_habr
        }
        ProviderKind::Medrxiv => {
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_medrxiv
        }
        ProviderKind::Reddit => {
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_reddit
        }
        ProviderKind::Twitter => {
            should_enable_error_prints = CONFIG.enable_error_prints.enable_error_prints_for_twitter
        }
    }
    let should_enable_cleaning_warning_logs_directory: bool;
    match value.4 {
        ProviderKind::Arxiv => {
            should_enable_cleaning_warning_logs_directory = CONFIG
                .enable_cleaning_warning_logs_directory
                .enable_cleaning_warning_logs_directory_for_arxiv
        }
        ProviderKind::Biorxiv => {
            should_enable_cleaning_warning_logs_directory = CONFIG
                .enable_cleaning_warning_logs_directory
                .enable_cleaning_warning_logs_directory_for_biorxiv
        }
        ProviderKind::Habr => {
            should_enable_cleaning_warning_logs_directory = CONFIG
                .enable_cleaning_warning_logs_directory
                .enable_cleaning_warning_logs_directory_for_habr
        }
        ProviderKind::Medrxiv => {
            should_enable_cleaning_warning_logs_directory = CONFIG
                .enable_cleaning_warning_logs_directory
                .enable_cleaning_warning_logs_directory_for_medrxiv
        }
        ProviderKind::Reddit => {
            should_enable_cleaning_warning_logs_directory = CONFIG
                .enable_cleaning_warning_logs_directory
                .enable_cleaning_warning_logs_directory_for_reddit
        }
        ProviderKind::Twitter => {
            should_enable_cleaning_warning_logs_directory = CONFIG
                .enable_cleaning_warning_logs_directory
                .enable_cleaning_warning_logs_directory_for_twitter
        }
    }
    if CONFIG.params.enable_all_cleaning_warning_logs_directory
        && should_enable_cleaning_warning_logs_directory
    {
        let path = format!(
            "logs/{}/{:?}",
            &CONFIG.params.warning_logs_directory_name, value.4
        );
        if Path::new(&path).is_dir() {
            let result_of_recursively_removing_warning_logs_directory = fs::remove_dir_all(&path);
            match result_of_recursively_removing_warning_logs_directory {
                Ok(_) => {
                    if should_enable_prints {
                        println!("folder {} has been deleted", &path);
                    }
                }
                Err(e) => {
                    if should_enable_error_prints {
                        let error_message =
                            format!("delete folder problem{} {}", &path, e.to_string());
                        print_error_red(file!().to_string(), line!().to_string(), error_message)
                    }
                }
            }
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
                    rss_write_error_logs_into_file(
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
                    rss_write_error_logs_into_file(
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
                    rss_write_error_logs_into_file(
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
                    rss_write_error_logs_into_file(
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
                rss_write_error_logs_into_file(
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
                rss_write_error_logs_into_file(
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
                rss_write_error_logs_into_file(
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
            rss_write_error_logs_into_file(
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
            rss_write_error_logs_into_file(
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
