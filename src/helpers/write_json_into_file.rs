use serde_json::Value;

use crate::fetch::write_error_logs_into_file_for_provider::write_error_logs_into_file_for_provider;
use crate::helpers::json_to_string::json_to_string;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn write_json_into_file(path_to_file: String, json_object: Value) -> bool {
    let option_stringified_json = json_to_string(json_object);
    if let Some(stringified_json) = option_stringified_json {
        write_error_logs_into_file_for_provider(path_to_file, stringified_json)
    } else {
        false
    }
}
