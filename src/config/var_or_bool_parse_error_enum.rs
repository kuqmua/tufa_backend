use core::str::ParseBoolError;
use std::env::VarError;

#[derive(Debug)]
pub enum VarOrBoolParseError {
    Var(VarError),
    Bool(ParseBoolError),
}
