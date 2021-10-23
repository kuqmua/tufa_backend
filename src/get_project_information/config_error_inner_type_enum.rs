use crate::get_project_information::var_or_bool_parse_error_enum::VarOrBoolParseError;
use crate::get_project_information::var_or_int_parse_error_enum::VarOrIntParseError;
use std::env::VarError;

#[derive(Debug)]
pub enum ConfigErrorInnerType {
    VarErrorHandle(VarError),
    VarOrBoolParseErrorHandle(VarOrBoolParseError),
    VarOrIntParseErrorErrorHandle(VarOrIntParseError),
}
