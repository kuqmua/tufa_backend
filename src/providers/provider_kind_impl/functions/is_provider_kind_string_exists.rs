use crate::providers::provider_kind_enum::ProviderKind;

use strum::IntoEnumIterator;

impl ProviderKind {
    pub fn is_provider_kind_string_exists(potential_provider_kind_string: &str) -> bool {
        for provider_kind in ProviderKind::iter() {
            let provider_kind_string = ProviderKind::get_string_name(provider_kind);
            if provider_kind_string == potential_provider_kind_string {
                return true;
            }
        }
        false
    }
}
