use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;

use crate::traits::get_env_name_trait::GetEnvName;

impl EnvBoolVar {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvBoolVar> {
        let mut hashmap: HashMap<&'static str, EnvBoolVar> =
            HashMap::with_capacity(EnvBoolVar::get_length());
        for env_var_kind in EnvBoolVar::iter() {
            hashmap.insert(env_var_kind.get_env_name(), env_var_kind);
        }
        hashmap
    }
}
