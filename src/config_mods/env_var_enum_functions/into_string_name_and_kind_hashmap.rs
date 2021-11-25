use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config_mods::env_var_enum::EnvVar;

impl EnvVar {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvVar> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<
            &'static str,
            EnvVar,
        > = HashMap::with_capacity(EnvVar::get_length());
        for env_var_name_kind_kind in EnvVar::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(
                EnvVar::get_env_name(env_var_name_kind_kind),
                env_var_name_kind_kind,
            );
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
}
