use crate::providers::provider_kind_enum::ProviderKind;

use crate::traits::enum_extention::EnumExtenstion;

use strum::IntoEnumIterator;

impl EnumExtenstion for ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_array() -> &'static [Self] {
        Self::all_variants()
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_vec() -> Vec<Self> {
        let mut self_vec = Vec::with_capacity(Self::get_length());
        for self_variant in Self::iter() {
            self_vec.push(self_variant);
        }
        self_vec
    }
}
