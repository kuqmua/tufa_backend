use std::env::VarError;
use core::num::ParseIntError;

#[derive(Debug)] 
pub enum VarOrIntParseError {
    Var(VarError),
    Int(ParseIntError)
}