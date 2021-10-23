use crate::get_project_information::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::get_project_information::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::get_project_information::config_values_types_enums::env_var_string_enum::EnvStringVar;
use crate::get_project_information::config_values_types_enums::env_var_u8_enum::EnvU8Var;

#[derive(Debug)]
pub enum EnvVarTypes {
    Bool(EnvBoolVar),
    String(EnvStringVar),
    U8(EnvU8Var),
    I64(EnvI64Var),
}
