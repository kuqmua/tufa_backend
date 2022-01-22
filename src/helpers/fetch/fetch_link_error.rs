use std::fmt;

use strum_macros::Display;

#[derive(Debug, ImplDisplayDerive)]
pub struct FetchLinkError {
    pub source: Box<FetchLinkErrorEnum>,
}

#[derive(Debug, Display)] //Debug only for prints
pub enum FetchLinkErrorEnum {
    ReqwestBlockingGet(reqwest::Error),
    StatusCode(reqwest::StatusCode),
    ResponseText(reqwest::Error),
}
