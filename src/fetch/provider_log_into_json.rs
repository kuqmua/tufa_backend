use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::providers::provider_kind_enum::ProviderKind;
use chrono::Local;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use serde_json::json;
use serde_json::Value;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn provider_log_into_json(
    link: &str,
    are_there_items: &AreThereItems,
    provider_kind: &ProviderKind,
) -> Option<Value> {
    match are_there_items {
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
        AreThereItems::NopeNoTag(tag) => Some(json!({
            "link": link,
            "tag": tag,
            "part_of": format!("{:?}", provider_kind),
            "date": Local::now().to_string()
        })),
    }
}
