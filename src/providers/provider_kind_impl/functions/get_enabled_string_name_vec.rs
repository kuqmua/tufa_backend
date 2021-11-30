use crate::providers::provider_kind_enum::ProviderKind;

use crate::traits::provider_kind_trait::ProviderKindTrait;

use strum::IntoEnumIterator;

impl ProviderKind {
    pub fn get_enabled_string_name_vec() -> Vec<String> {
        let mut string_name_vec: Vec<String> = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in
            ProviderKind::iter().filter(|element| element.is_enabled())
        {
            string_name_vec.push(format!("{}", provider_kind));
        }
        string_name_vec
    }
}
