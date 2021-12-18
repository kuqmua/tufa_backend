use std::collections::HashMap;

use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

use crate::traits::enum_extention::EnumExtenstion;

use strum::IntoEnumIterator;

use convert_case::{Case, Casing};

impl EnumExtenstion for EnvU8Var {
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
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_variant_hashmap() -> HashMap<String, Self> {
        let mut variants_hashmap: HashMap<String, Self> =
            HashMap::with_capacity(Self::get_length());
        for variant in Self::iter() {
            variants_hashmap.insert(format!("{}", variant), variant);
        }
        variants_hashmap
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_variant_tuple_vec() -> Vec<(String, Self)> {
        let mut variants_vec = Vec::with_capacity(Self::get_length());
        for variant in Self::iter() {
            variants_vec.push((format!("{}", variant), variant));
        }
        variants_vec
    }
    fn to_upper_snake_case(&self) -> String {
        format!("{:?}", self).to_case(Case::Snake)
    }
}
