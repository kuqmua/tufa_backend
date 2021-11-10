use serde_json::Value;

use crate::helpers::write_string_into_file::write_string_into_file;

#[derive(Debug)]
pub enum WriteJsonIntoFileError {
    SerdeJsonError(serde_json::Error),
    StdIoError(std::io::Error),
}
impl From<serde_json::Error> for WriteJsonIntoFileError {
    fn from(e:serde_json::Error) -> Self {
        WriteJsonIntoFileError::SerdeJsonError(e)
    }
}
impl From<std::io::Error> for WriteJsonIntoFileError {
    fn from(e: std::io::Error) -> Self {
        WriteJsonIntoFileError::StdIoError(e)
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn write_json_into_file(path_to_file: String, json_object: Value) -> Result<(), WriteJsonIntoFileError> {
    let stringified_json = serde_json::to_string_pretty(&json_object)?;
    Ok(write_string_into_file(path_to_file, stringified_json)?)
}
