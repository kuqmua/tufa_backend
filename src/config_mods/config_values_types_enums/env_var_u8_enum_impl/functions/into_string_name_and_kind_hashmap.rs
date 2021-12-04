use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

use crate::traits::env_var_trait::EnvVarTrait;

impl EnvU8Var {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvU8Var> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<
            &'static str,
            EnvU8Var,
        > = HashMap::with_capacity(EnvU8Var::get_length());
        for env_var_name_kind_kind in EnvU8Var::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(
                env_var_name_kind_kind.get_env_name(),
                env_var_name_kind_kind,
            );
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
}
