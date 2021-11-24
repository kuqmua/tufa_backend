use std::path::Path;
use std::fs;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind_enum::CleanLogsDirError;

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn remove_logs_directory(provider_kind: ProviderKind) -> Result<(), CleanLogsDirError> {
        let path = ProviderKind::get_path_to_logs_directory(provider_kind);
        if !Path::new(&path).is_dir() {
            return Err(CleanLogsDirError::PathIsNotDir { path });
        }
        fs::remove_dir_all(&path)?;
        Ok(())
    }
}