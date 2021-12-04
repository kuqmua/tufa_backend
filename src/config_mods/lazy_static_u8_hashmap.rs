extern crate toml;

use std::collections::HashMap;

use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

lazy_static! {
    pub static ref U8_ENV_VARS_HASHMAP: HashMap<EnvU8Var, u8> =
        EnvU8Var::get_env_values_hashmap().expect("Cannot get u8 config values");
}
