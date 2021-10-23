use crate::config::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::config::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config::config_values_types_enums::env_var_string_enum::EnvStringVar;
use crate::config::config_values_types_enums::env_var_u8_enum::EnvU8Var;

#[derive(Debug)]
pub enum ConfigEnvVarErrorType {
    Bool(EnvBoolVar),
    String(EnvStringVar),
    U8(EnvU8Var),
    I64(EnvI64Var),
}
