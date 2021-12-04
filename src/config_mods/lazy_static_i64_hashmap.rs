extern crate toml;

use std::collections::HashMap;

use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;

use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

lazy_static! {
    pub static ref I64_ENV_VARS_HASHMAP: HashMap<EnvI64Var, i64> =
        EnvI64Var::get_env_values_hashmap().expect("Cannot get i64 config values");
}
