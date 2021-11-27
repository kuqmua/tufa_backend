use strum::IntoEnumIterator;

use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;

impl EnvBoolVar {
    pub fn into_vec() -> Vec<EnvBoolVar> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvBoolVar::get_length());
        for env_var_name_kind in EnvBoolVar::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
}
