use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind_enum::CleanLogsDirError;

use std::collections::HashMap;

use strum::IntoEnumIterator;

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn remove_providers_logs_directories(
    ) -> Result<(), HashMap<ProviderKind, CleanLogsDirError>> {
        let mut result_hashmap: HashMap<ProviderKind, CleanLogsDirError> =
            HashMap::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter()
            .filter(|element| ProviderKind::is_cleaning_warning_logs_directory_enable(*element))
        {
            if let Err(e) = ProviderKind::remove_logs_directory(provider_kind) {
                result_hashmap.insert(provider_kind, e);
            }
        }
        if result_hashmap.is_empty() {
            Ok(())
        } else {
            Err(result_hashmap)
        }
    }
}