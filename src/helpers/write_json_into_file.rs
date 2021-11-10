use serde_json::Value;

use crate::helpers::write_string_into_file::write_string_into_file;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn write_json_into_file(path_to_file: String, json_object: Value) -> bool {
    let result_stringified_json = serde_json::to_string_pretty(&json_object);
    match result_stringified_json {
        Ok(stringified_json) => {
            write_string_into_file(path_to_file, stringified_json)
        },
        Err(e) => {
            false
        }
    }
}
