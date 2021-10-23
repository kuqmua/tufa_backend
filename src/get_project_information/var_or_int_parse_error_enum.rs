use core::num::ParseIntError;
use std::env::VarError;

#[derive(Debug)]
pub enum VarOrIntParseError {
    Var(VarError),
    Int(ParseIntError),
}
