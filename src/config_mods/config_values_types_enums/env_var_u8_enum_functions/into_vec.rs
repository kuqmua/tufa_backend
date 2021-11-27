use strum::IntoEnumIterator;

use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

impl EnvU8Var {
    pub fn into_vec() -> Vec<EnvU8Var> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvU8Var::get_length());
        for env_var_name_kind in EnvU8Var::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
}
