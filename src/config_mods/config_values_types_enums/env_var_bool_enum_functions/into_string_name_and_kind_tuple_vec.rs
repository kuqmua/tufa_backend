use strum::IntoEnumIterator;

use crate::{
    config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar,
    traits::get_env_name_trait::GetEnvName,
};

impl EnvBoolVar {
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvBoolVar)> {
        let mut vec = Vec::with_capacity(EnvBoolVar::get_length());
        for env_var_kind in EnvBoolVar::iter() {
            vec.push((env_var_kind.get_env_name(), env_var_kind));
        }
        vec
    }
}
