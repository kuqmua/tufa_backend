use std::path::Path;

use serde_json::Value;

use crate::helpers::write_string_into_file::write_string_into_file;

#[derive(thiserror::Error, Debug)]
pub enum WriteJsonIntoFileError {
    #[error("serde_json::to_string_pretty serde_json::Error error: `{0}`.")]
    SerdeJsonError(
        #[from]
        #[source]
        serde_json::Error,
    ),
    #[error("write_string_into_file std::io::Error error: `{0}`.")]
    StdIoError(
        #[from]
        #[source]
        std::io::Error,
    ),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn write_json_into_file(path: &Path, json_object: Value) -> Result<(), WriteJsonIntoFileError> {
    let stringified_json = serde_json::to_string_pretty(&json_object)?;
    Ok(write_string_into_file(path, stringified_json)?)
}
