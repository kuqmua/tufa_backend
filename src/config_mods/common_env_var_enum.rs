use crate::traits::enum_extention::EnumExtenstion;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;
use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

use strum_macros::Display;
use strum_macros::EnumIter;

use convert_case::{Case, Casing};

use std::collections::HashMap;

use strum::IntoEnumIterator;

#[derive(
    EnumExtenstion,
    EnumIter,
    Debug,
    Display,
)]
pub enum CommonEnvVar {
    EnvBoolVar,
    EnvI64Var,
    EnvStringVar,
    EnvU8Var
}

impl CommonEnvVar {
    pub fn check_valid_env_vars_type(&self) {
        match self {
            CommonEnvVar::EnvBoolVar => EnvBoolVar::check_valid_typed_env_vars(),
            CommonEnvVar::EnvI64Var => EnvI64Var::check_valid_typed_env_vars(),
            CommonEnvVar::EnvStringVar => EnvStringVar::check_valid_typed_env_vars(),
            CommonEnvVar::EnvU8Var => EnvU8Var::check_valid_typed_env_vars(),
        }
    }
}
