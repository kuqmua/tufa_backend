use serde_json::Value;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn json_to_string(json_object: Value) -> Option<String> {
    let result_of_stringify_json_object_pretty = serde_json::to_string_pretty(&json_object);
    match result_of_stringify_json_object_pretty {
        Ok(string_json_prettyfied) => Some(string_json_prettyfied),
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "error cast json into string {} {}",
                    &json_object,
                    e.to_string()
                ),
            );
            None
        }
    }
}
