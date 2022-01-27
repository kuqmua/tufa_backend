use std::fmt;

#[derive(Debug, ImplDisplayDerive)]
pub struct FetchLinkError {
    pub source: Box<FetchLinkErrorEnum>,
    pub line: String,
}

#[derive(Debug)]
pub enum FetchLinkErrorEnum {
    ReqwestBlockingGet(reqwest::Error),
    StatusCode(reqwest::StatusCode),
    ResponseText(reqwest::Error),
}
