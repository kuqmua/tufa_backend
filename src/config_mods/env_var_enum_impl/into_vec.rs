use crate::config_mods::env_var_enum::EnvVar;

use strum::IntoEnumIterator;

impl EnvVar {
    pub fn into_vec() -> Vec<EnvVar> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvVar::get_length());
        for env_var_name_kind in EnvVar::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
}
