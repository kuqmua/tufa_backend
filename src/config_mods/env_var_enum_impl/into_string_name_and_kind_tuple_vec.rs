use crate::config_mods::env_var_enum::EnvVar;

use strum::IntoEnumIterator;

impl EnvVar {
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvVar)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvVar::get_length());
        for env_var_name_kind in EnvVar::iter() {
            env_var_name_kind_vec
                .push((EnvVar::get_env_name(env_var_name_kind), env_var_name_kind));
        }
        env_var_name_kind_vec
    }
}
