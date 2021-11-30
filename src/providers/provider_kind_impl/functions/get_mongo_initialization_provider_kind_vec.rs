use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::providers::provider_kind_enum::ProviderKind;

use strum::IntoEnumIterator;

impl ProviderKind {
    pub fn get_mongo_initialization_provider_kind_vec() -> Vec<ProviderKind> {
        let mut vec_of_filtered_provider_names: Vec<ProviderKind> =
            Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter()
            .filter(|provider_kind| provider_kind.is_mongo_initialization_enabled())
        {
            vec_of_filtered_provider_names.push(provider_kind)
        }
        vec_of_filtered_provider_names
    }
}
