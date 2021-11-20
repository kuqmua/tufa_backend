use std::io::Error;
use std::{fs::File, io::Write};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn write_string_into_file(path_to_file: String, stringified_json: String) -> Result<(), Error> {
    let path = std::path::Path::new(&path_to_file);
    if let Some(prefix) = path.parent() {
        std::fs::create_dir_all(prefix)?;
    }
    let mut log_file = File::create(path)?;
    log_file.write_all(stringified_json.as_bytes())?;
    log_file.sync_all()?;
    Ok(())
}
