use serde_json::Value;

use helpers_lib::json_to_string::json_to_string;
use helpers_lib::write_string_into_file::write_string_into_file;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn write_json_into_file(path_to_file: String, json_object: Value) -> bool {
    let option_stringified_json = json_to_string(json_object);
    if let Some(stringified_json) = option_stringified_json {
        write_string_into_file(path_to_file, stringified_json)
    } else {
        false
    }
}
