use std::path::Path;

use serde_json::Value;

use crate::helpers::write_string_into_file::write_string_into_file;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WriteJsonIntoFileError {
    #[error("SerdeJsonError message")]
    SerdeJsonError(
        #[from]
        #[source]
        serde_json::Error,
    ),
    #[error("StdIoError message")]
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
