use crate::providers::provider_kind_enum::ProviderKind;

use strum::IntoEnumIterator;

impl ProviderKind {
    pub fn get_mongo_initialization_string_name_vec() -> Vec<&'static str> {
        let mut vec_of_filtered_provider_names: Vec<&'static str> =
            Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter()
            .filter(|element| ProviderKind::is_mongo_initialization_enabled(*element))
        {
            vec_of_filtered_provider_names.push(ProviderKind::get_string_name(provider_kind))
        }
        vec_of_filtered_provider_names
    }
}
