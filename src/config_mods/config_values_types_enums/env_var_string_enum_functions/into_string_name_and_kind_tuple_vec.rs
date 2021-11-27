use strum::IntoEnumIterator;

use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;

impl EnvStringVar {
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvStringVar)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvStringVar::get_length());
        for env_var_name_kind in EnvStringVar::iter() {
            env_var_name_kind_vec.push((
                EnvStringVar::get_env_name(env_var_name_kind),
                env_var_name_kind,
            ));
        }
        env_var_name_kind_vec
    }
}
