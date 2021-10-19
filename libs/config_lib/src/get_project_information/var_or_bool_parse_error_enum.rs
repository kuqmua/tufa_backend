use std::env::VarError;
use core::str::ParseBoolError;

#[derive(Debug)] 
pub enum VarOrBoolParseError {
    Var(VarError),
    Bool(ParseBoolError)
}