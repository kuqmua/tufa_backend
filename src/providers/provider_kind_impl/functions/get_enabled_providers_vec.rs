use crate::providers::provider_kind_enum::ProviderKind;

use crate::traits::provider_kind_trait::ProviderKindTrait;

use strum::IntoEnumIterator;

impl ProviderKind {
    pub fn get_enabled_providers_vec() -> Vec<ProviderKind> {
        let mut providers_vec: Vec<ProviderKind> = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in
            ProviderKind::iter().filter(|provider_kind| provider_kind.is_enabled())
        {
            providers_vec.push(provider_kind);
        }
        providers_vec
    }
}
