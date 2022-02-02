use std::fmt;

#[derive(Debug, ImplDisplayDerive)]
pub struct FetchLinkError {
    pub source: Box<FetchLinkErrorEnum>,
}

#[derive(Debug)]
pub enum FetchLinkErrorEnum {
    ReqwestBlockingGet {
        source: reqwest::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    StatusCode {
        source: reqwest::StatusCode,
        file: &'static str,
        line: u32,
        column: u32,
    },
    ResponseText {
        source: reqwest::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
}
