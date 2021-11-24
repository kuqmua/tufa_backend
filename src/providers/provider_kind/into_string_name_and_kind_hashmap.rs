use std::collections::HashMap;

use crate::providers::provider_kind_enum::ProviderKind;

use strum::IntoEnumIterator;

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, ProviderKind> {
        //its String coz legacy
        let mut config_provider_string_to_enum_struct_hasmap: HashMap<&'static str, ProviderKind> =
            HashMap::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter() {
            config_provider_string_to_enum_struct_hasmap
                .insert(ProviderKind::get_string_name(provider_kind), provider_kind);
        }
        config_provider_string_to_enum_struct_hasmap
    }
}