use std::fs;
use std::path::Path;

use crate::providers::provider_kind_enum::CleanLogsDirError;
use crate::providers::provider_kind_enum::ProviderKind;

use crate::traits::provider_kind_trait::ProviderKindTrait;

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn remove_logs_directory(provider_kind: ProviderKind) -> Result<(), CleanLogsDirError> {
        let path = provider_kind.get_path_to_logs_directory();
        if !Path::new(&path).is_dir() {
            return Err(CleanLogsDirError::PathIsNotDir { path });
        }
        fs::remove_dir_all(&path)?;
        Ok(())
    }
}
