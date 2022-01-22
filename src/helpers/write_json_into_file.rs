use std::path::Path;

use serde_json::Value;

use crate::helpers::write_string_into_file::write_string_into_file_with_tokio;
use crate::helpers::write_string_into_file::WriteStringIntoFileWithTokioError;

#[derive(thiserror::Error, Debug)]
pub enum WriteJsonIntoFileError {
    #[error("serde_json::to_string_pretty serde_json::Error error: `{0}`.")]
    SerdeJsonError(
        #[from]
        #[source]
        serde_json::Error,
    ),
    #[error("write_string_into_file_with_tokio std::io::Error error: `{0}`.")]
    StdIoError(
        #[from]
        #[source]
        WriteStringIntoFileWithTokioError,
    ),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn write_json_into_file(path: &Path, json_object: Value) -> Result<(), WriteJsonIntoFileError> {
    let stringified_json = serde_json::to_string_pretty(&json_object)?;
    Ok(write_string_into_file_with_tokio(path, stringified_json).await?)
}
