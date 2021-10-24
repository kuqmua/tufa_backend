use crate::fetch::rss_handle_error_status_code::handle_error_status_code;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::providers::provider_kind_enum::ProviderKind;
use chrono::Local;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use serde_json::json;
use serde_json::Value;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn provider_log_into_json(
    link: &str,
    unhandled_fetch_status_info: &UnhandledFetchStatusInfo,
    handled_fetch_status_info: &HandledFetchStatusInfo,
    are_there_items: &AreThereItems,
    provider_kind: &ProviderKind,
) -> Option<Value> {
    match unhandled_fetch_status_info {
        UnhandledFetchStatusInfo::Success => match handled_fetch_status_info {
            HandledFetchStatusInfo::Success => match are_there_items {
                AreThereItems::Yep => {
                    print_colorful_message(
                    Some(provider_kind),
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    "UnhandledFetchStatusInfo::Success, HandledFetchStatusInfo::Success, AreThereItems::Yep --- its not suppose to happend".to_string(),
                );
                    None
                }
                //todo: field names into config?
                AreThereItems::Initialized => Some(json!({
                    "link": link,
                    "part_of": format!("{:?}", provider_kind),
                    "date": Local::now().to_string()
                })),
                AreThereItems::NopeButThereIsTag(fetch_result_string) => {
                    //"</item>" tag
                    Some(json!({
                        "link": link,
                        "fetch_result_string": fetch_result_string,
                        "part_of": format!("{:?}", provider_kind),
                        "date": Local::now().to_string()
                    }))
                }
                AreThereItems::ConversionFromStrError(fetch_result_string, error) => Some(json!({
                    "link": link,
                    "fetch_result_string": fetch_result_string,
                    "error": error,
                    "part_of": format!("{:?}", provider_kind),
                    "date": Local::now().to_string()
                })),
            },
            HandledFetchStatusInfo::Initialized => Some(json!({
                "link": link,
                "part_of": format!("{:?}", provider_kind),
                "date": Local::now().to_string()
            })),
            HandledFetchStatusInfo::ResToTextError(error) => Some(json!({
                "link": link,
                "error": error,
                "part_of": format!("{:?}", provider_kind),
                "date": Local::now().to_string()
            })),
            HandledFetchStatusInfo::ResStatusError(status_code) => {
                handle_error_status_code(*status_code, link); //todo: maybe its not a place to handle status code?
                Some(json!({
                    "link": link,
                    "status_code": status_code.to_string(),
                    "part_of": format!("{:?}", provider_kind),
                    "date": Local::now().to_string()
                }))
            }
        },
        UnhandledFetchStatusInfo::Initialized => Some(json!({
            "link": link,
            "part_of": format!("{:?}", provider_kind),
            "date": Local::now().to_string()
        })),
        UnhandledFetchStatusInfo::Failure(box_dyn_error) => Some(json!({
            "link": link,
            "error": box_dyn_error,
            "part_of": format!("{:?}", provider_kind),
            "date": Local::now().to_string()
        })),
    }
}
