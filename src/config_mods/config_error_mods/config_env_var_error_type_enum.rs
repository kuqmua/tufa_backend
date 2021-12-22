use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;
use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

#[derive(Debug)]
pub enum ConfigEnvVarErrorType {
    EnvBoolVar(EnvBoolVar),
    EnvStringVar(EnvStringVar),
    EnvU8Var(EnvU8Var),
    EnvI64Var(EnvI64Var),
}
