use std::io::Error;
use std::{fs::File, io::Write};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn write_string_into_file(file_name: String, stringified_json: String) -> Result<(), Error> {
    let mut log_file = File::create(&file_name)?;
    log_file.write_all(stringified_json.as_bytes())?;
    Ok(())
}
