use std::fmt;

#[derive(Debug, ImplDisplayDerive)]
pub struct FetchLinkError {
    pub source: Box<FetchLinkErrorEnum>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub enum FetchLinkErrorEnum {
    ReqwestBlockingGet(reqwest::Error),
    StatusCode(reqwest::StatusCode),
    ResponseText(reqwest::Error),
}
