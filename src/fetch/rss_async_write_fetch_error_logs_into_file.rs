use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_handle_error_status_code::handle_error_status_code;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::fetch::rss_write_error_logs_into_file::rss_write_error_logs_into_file;
use chrono::Local;
use serde_json::json;
use std::time::Instant;

#[allow(clippy::clippy::too_many_arguments)]
pub async fn rss_async_write_fetch_error_logs_into_file(
    value: (
        CommonRssPostStruct,
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    ),
    unhandled_success_handled_success_are_there_items_initialized_posts_dir: &str,
    provider_kind: &ProviderKind,
    enable_prints: bool,
    enable_error_prints: bool,
    enable_time_measurement: bool,
    time: Instant,
    warning_logs_directory_name: &str,
) {
    match value.2 {
        UnhandledFetchStatusInfo::Success => match value.3 {
            HandledFetchStatusInfo::Success => match value.4 {
                AreThereItems::Yep => (),
                AreThereItems::Initialized => {
                    let json_object = json!({
                        "link": value.1,
                        "part_of": format!("{:?}", provider_kind),
                        "date": Local::now().to_string()
                    });
                    rss_write_error_logs_into_file(
                        json_object,
                        &provider_kind,
                        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                        enable_prints,
                        enable_error_prints,
                        warning_logs_directory_name,
                    )
                }
                AreThereItems::NopeButThereIsTag(fetch_result_string) => {
                    //"</item>" tag
                    let json_object = json!({
                        "link": value.1,
                        "fetch_result_string": fetch_result_string,
                        "part_of": format!("{:?}", provider_kind),
                        "date": Local::now().to_string()
                    });
                    rss_write_error_logs_into_file(
                        json_object,
                        &provider_kind,
                        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                        enable_prints,
                        enable_error_prints,
                        warning_logs_directory_name,
                    )
                }
                AreThereItems::ConversionFromStrError(fetch_result_string, error) => {
                    let json_object = json!({
                        "link": value.1,
                        "fetch_result_string": fetch_result_string,
                        "error": error,
                        "part_of": format!("{:?}", provider_kind),
                        "date": Local::now().to_string()
                    });
                    rss_write_error_logs_into_file(
                        json_object,
                        &provider_kind,
                        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                        enable_prints,
                        enable_error_prints,
                        warning_logs_directory_name,
                    )
                }
                AreThereItems::NopeNoTag(fetch_result_string) => {
                    let json_object = json!({
                        "link": value.1,
                        "page_content": fetch_result_string,
                        "part_of": format!("{:?}", provider_kind),
                        "date": Local::now().to_string()
                    });
                    rss_write_error_logs_into_file(
                        json_object,
                        &provider_kind,
                        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                        enable_prints,
                        enable_error_prints,
                        warning_logs_directory_name,
                    )
                }
            },
            HandledFetchStatusInfo::Initialized => {
                let json_object = json!({
                    "link": value.1,
                    "part_of": format!("{:?}", provider_kind),
                    "date": Local::now().to_string()
                });
                rss_write_error_logs_into_file(
                    json_object,
                    &provider_kind,
                    unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                    enable_prints,
                    enable_error_prints,
                    warning_logs_directory_name,
                )
            }
            HandledFetchStatusInfo::ResToTextError(error) => {
                let json_object = json!({
                    "link": value.1,
                    "error": error,
                    "part_of": format!("{:?}", provider_kind),
                    "date": Local::now().to_string()
                });
                rss_write_error_logs_into_file(
                    json_object,
                    &provider_kind,
                    unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                    enable_prints,
                    enable_error_prints,
                    warning_logs_directory_name,
                )
            }
            HandledFetchStatusInfo::ResStatusError(status_code) => {
                let json_object = json!({
                    "link": value.1,
                    "status_code": status_code.to_string(),
                    "part_of": format!("{:?}", provider_kind),
                    "date": Local::now().to_string()
                });
                handle_error_status_code(status_code, value.1);
                rss_write_error_logs_into_file(
                    json_object,
                    &provider_kind,
                    unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                    enable_prints,
                    enable_error_prints,
                    warning_logs_directory_name,
                )
            }
        },
        UnhandledFetchStatusInfo::Initialized => {
            let json_object = json!({
                "link": value.1,
                "part_of": format!("{:?}", provider_kind),
                "date": Local::now().to_string()
            });
            rss_write_error_logs_into_file(
                json_object,
                &provider_kind,
                unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                enable_prints,
                enable_error_prints,
                warning_logs_directory_name,
            )
        }
        UnhandledFetchStatusInfo::Failure(box_dyn_error) => {
            let json_object = json!({
                "link": value.1,
                "error": box_dyn_error,
                "part_of": format!("{:?}", provider_kind),
                "date": Local::now().to_string()
            });
            rss_write_error_logs_into_file(
                json_object,
                &provider_kind,
                unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                enable_prints,
                enable_error_prints,
                warning_logs_directory_name,
            )
        }
    }
    if enable_time_measurement {
        println!(
            "write fetch error logs into file done in {} seconds {} miliseconds for {:#?}",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
            provider_kind
        );
    }
}
