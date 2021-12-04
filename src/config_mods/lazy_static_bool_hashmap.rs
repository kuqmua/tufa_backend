extern crate toml;

use std::collections::HashMap;

use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;

use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

lazy_static! {
    pub static ref BOOL_ENV_VARS_HASHMAP: HashMap<EnvBoolVar, bool> =
        EnvBoolVar::get_env_values_hashmap().expect("Cannot get bool config values");
}
