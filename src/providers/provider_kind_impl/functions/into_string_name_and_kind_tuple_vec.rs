use crate::providers::provider_kind_enum::ProviderKind;

use strum::IntoEnumIterator;

impl ProviderKind {
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(String, ProviderKind)> {
        let mut provider_kind_vec = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter() {
            provider_kind_vec.push((format!("{}", provider_kind), provider_kind));
        }
        provider_kind_vec
    }
}
