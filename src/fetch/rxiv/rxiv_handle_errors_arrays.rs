use crate::fetch::handle_error_status_code::handle_error_status_code;
use crate::fetch::provider_kind_enum::ProviderKind;
use crate::fetch::rxiv::metainfo_fetch_structures::AreThereItems;
use crate::fetch::rxiv::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rxiv::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rxiv::rxiv_structures::RxivPostStruct;
use crate::fetch::rxiv::rxiv_write_error_logs_into_file::rxiv_write_error_logs_into_file;
// use crate::overriding::prints::print_warning_yellow;
use chrono::Local;
use serde_json::json;
use std::collections::HashMap;

#[allow(clippy::clippy::too_many_arguments)]
pub fn rxiv_handle_errors_arrays(
    provider_kind: ProviderKind,
    enable_prints: bool,
    // enable_warning_prints: bool,
    enable_error_prints: bool,
    some_error_posts: HashMap<
        String,
        (
            RxivPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
            ProviderKind,
        ),
    >,
) {
    let unhandled_success_handled_success_are_there_items_initialized_posts_dir =
        "unhandled_success_handled_success_are_there_items_initialized_posts";
    for (key, value) in some_error_posts {
        match value.2 {
            UnhandledFetchStatusInfo::Success => match value.3 {
                HandledFetchStatusInfo::Success => match value.4 {
                    AreThereItems::Yep => (),
                    AreThereItems::Initialized => {
                        let json_object = json!({
                            "topic": key,
                            "link": value.1,
                            "part_of": format!("{:?}", provider_kind),
                            "date": Local::now().to_string()
                        });
                        rxiv_write_error_logs_into_file(
                            json_object,
                            &provider_kind,
                            unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                            enable_prints,
                            enable_error_prints,
                            key,
                        )
                    }
                    AreThereItems::NopeButThereIsTag(fetch_result_string) => {
                        //"</item>" tag
                        let json_object = json!({
                            "topic": key,
                            "link": value.1,
                            "fetch_result_string": fetch_result_string,
                            "part_of": format!("{:?}", provider_kind),
                            "date": Local::now().to_string()
                        });
                        rxiv_write_error_logs_into_file(
                            json_object,
                            &provider_kind,
                            unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                            enable_prints,
                            enable_error_prints,
                            key,
                        )
                    }
                    AreThereItems::ConversionFromStrError(fetch_result_string, error) => {
                        let json_object = json!({
                            "topic": key,
                            "link": value.1,
                            "fetch_result_string": fetch_result_string,
                            "error": error,
                            "part_of": format!("{:?}", provider_kind),
                            "date": Local::now().to_string()
                        });
                        rxiv_write_error_logs_into_file(
                            json_object,
                            &provider_kind,
                            unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                            enable_prints,
                            enable_error_prints,
                            key,
                        )
                    }
                    AreThereItems::NopeNoTag(fetch_result_string) => {
                        let json_object = json!({
                            "topic": key,
                            "link": value.1,
                            "page_content": fetch_result_string,
                            "part_of": format!("{:?}", provider_kind),
                            "date": Local::now().to_string()
                        });
                        rxiv_write_error_logs_into_file(
                            json_object,
                            &provider_kind,
                            unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                            enable_prints,
                            enable_error_prints,
                            key,
                        )
                    }
                },
                HandledFetchStatusInfo::Initialized => {
                    let json_object = json!({
                        "topic": key,
                        "link": value.1,
                        "part_of": format!("{:?}", provider_kind),
                        "date": Local::now().to_string()
                    });
                    rxiv_write_error_logs_into_file(
                        json_object,
                        &provider_kind,
                        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                        enable_prints,
                        enable_error_prints,
                        key,
                    )
                }
                HandledFetchStatusInfo::ResToTextError(error) => {
                    let json_object = json!({
                        "topic": key,
                        "link": value.1,
                        "error": error,
                        "part_of": format!("{:?}", provider_kind),
                        "date": Local::now().to_string()
                    });
                    rxiv_write_error_logs_into_file(
                        json_object,
                        &provider_kind,
                        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                        enable_prints,
                        enable_error_prints,
                        key,
                    )
                }
                HandledFetchStatusInfo::ResStatusError(status_code) => {
                    let json_object = json!({
                        "topic": key,
                        "link": value.1,
                        "status_code": status_code.to_string(),
                        "part_of": format!("{:?}", provider_kind),
                        "date": Local::now().to_string()
                    });
                    handle_error_status_code(status_code, &key, value.1);
                    rxiv_write_error_logs_into_file(
                        json_object,
                        &provider_kind,
                        unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                        enable_prints,
                        enable_error_prints,
                        key,
                    )
                }
            },
            UnhandledFetchStatusInfo::Initialized => {
                let json_object = json!({
                    "topic": key,
                    "link": value.1,
                    "part_of": format!("{:?}", provider_kind),
                    "date": Local::now().to_string()
                });
                rxiv_write_error_logs_into_file(
                    json_object,
                    &provider_kind,
                    unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                    enable_prints,
                    enable_error_prints,
                    key,
                )
            }
            UnhandledFetchStatusInfo::Failure(box_dyn_error) => {
                let json_object = json!({
                    "topic": key,
                    "link": value.1,
                    "error": box_dyn_error,
                    "part_of": format!("{:?}", provider_kind),
                    "date": Local::now().to_string()
                });
                rxiv_write_error_logs_into_file(
                    json_object,
                    &provider_kind,
                    unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                    enable_prints,
                    enable_error_prints,
                    key,
                )
            }
        }
    }
}
