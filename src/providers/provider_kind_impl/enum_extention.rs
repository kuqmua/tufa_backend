use crate::providers::provider_kind_enum::ProviderKind;

use crate::traits::enum_extention::EnumExtenstion;

impl EnumExtenstion for ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_array() -> &'static [Self] {
        Self::all_variants()
    }
}
