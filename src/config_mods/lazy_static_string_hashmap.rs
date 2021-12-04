extern crate toml;

use std::collections::HashMap;

use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;

use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

lazy_static! {
    pub static ref STRING_ENV_VARS_HASHMAP: HashMap<EnvStringVar, String> =
        EnvStringVar::get_env_values_hashmap().expect("Cannot get string config values");
}
