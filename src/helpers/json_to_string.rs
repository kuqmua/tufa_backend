use serde_json::Value;
use serde_json::Error;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn json_to_string(json_object: Value) -> Result<String, Error> {
    let result_of_stringify_json_object_pretty = serde_json::to_string_pretty(&json_object);
    match result_of_stringify_json_object_pretty {
        Ok(string_json_prettyfied) => Ok(string_json_prettyfied),
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "serde_json::to_string_pretty failed, json_object: {} error: {:#?}",
                    &json_object,
                    e
                ),
            );
            Err(e)
        }
    }
}
