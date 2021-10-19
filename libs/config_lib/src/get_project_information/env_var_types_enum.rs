use crate::get_project_information::env_var_bool_enum::EnvBoolVar;
use crate::get_project_information::env_var_string_enum::EnvStringVar;
use crate::get_project_information::env_var_u8_enum::EnvU8Var;
use crate::get_project_information::env_var_i64_enum::EnvI64Var;

#[derive(Debug)]
pub enum EnvVarTypes {
    Bool(EnvBoolVar),
    String(EnvStringVar),
    U8(EnvU8Var),
    I64(EnvI64Var)
}