use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::providers::provider_kind_enum::ProviderKind;
use chrono::Local;

use serde_json::json;
use serde_json::Value;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn provider_log_into_json(
    link: &str,
    no_items_error: &NoItemsError,
    provider_kind: &ProviderKind,
) -> Value {
    match no_items_error {
        NoItemsError::ThereIsTag(fetch_result_string) => {
            //"</item>" tag
            json!({
                "link": link,
                "fetch_result_string": fetch_result_string,
                "part_of": ProviderKind::get_string_name(*provider_kind),
                "date": Local::now().to_string()
            })
        }
        NoItemsError::ConversionFromStrError(fetch_result_string, error) => json!({
            "link": link,
            "fetch_result_string": fetch_result_string,
            "error": error,
            "part_of": ProviderKind::get_string_name(*provider_kind),
            "date": Local::now().to_string()
        }),
        NoItemsError::NoTag(tag) => json!({
            "link": link,
            "tag": tag,
            "part_of": ProviderKind::get_string_name(*provider_kind),
            "date": Local::now().to_string()
        }),
    }
}
