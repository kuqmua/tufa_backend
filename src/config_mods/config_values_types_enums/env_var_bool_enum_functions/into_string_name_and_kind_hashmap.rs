use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;

impl EnvBoolVar {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvBoolVar> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<
            &'static str,
            EnvBoolVar,
        > = HashMap::with_capacity(EnvBoolVar::get_length());
        for env_var_name_kind_kind in EnvBoolVar::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(
                EnvBoolVar::get_env_name(env_var_name_kind_kind),
                env_var_name_kind_kind,
            );
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
}
