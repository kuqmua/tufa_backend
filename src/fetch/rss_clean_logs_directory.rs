use crate::providers::provider_kind_enum::ProviderKind;

use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum CleanLogsDirError {
    PathIsNotDir { path: String },
    RemoveDirAll { error: std::io::Error},
}
impl From<String> for CleanLogsDirError {
    fn from(e: String) -> Self {
        CleanLogsDirError::PathIsNotDir { path: e }
    }
}
impl From<std::io::Error> for CleanLogsDirError {
    fn from(e: std::io::Error) -> Self {
        CleanLogsDirError::RemoveDirAll { error: e }
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_clean_logs_directory(provider_kind: ProviderKind) -> Result<(), CleanLogsDirError> {
    let path = ProviderKind::get_path_to_logs_directory(provider_kind);
    if !Path::new(&path).is_dir() {
        return Err(CleanLogsDirError::PathIsNotDir { path });
    }
    fs::remove_dir_all(&path)?;
    Ok(())
}
