use crate::providers::provider_kind_enum::ProviderKind;

use strum::IntoEnumIterator;

impl ProviderKind {
    pub fn get_enabled_string_name_vec() -> Vec<&'static str> {
        let mut string_name_vec: Vec<&'static str> = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in
            ProviderKind::iter().filter(|element| ProviderKind::is_enabled(*element))
        {
            string_name_vec.push(ProviderKind::get_string_name(provider_kind));
        }
        string_name_vec
    }
}