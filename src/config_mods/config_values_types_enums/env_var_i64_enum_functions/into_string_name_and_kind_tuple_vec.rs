use strum::IntoEnumIterator;

use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;

use crate::traits::get_env_name_trait::GetEnvName;

impl EnvI64Var {
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvI64Var)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvI64Var::get_length());
        for env_var_name_kind in EnvI64Var::iter() {
            env_var_name_kind_vec.push((env_var_name_kind.get_env_name(), env_var_name_kind));
        }
        env_var_name_kind_vec
    }
}
