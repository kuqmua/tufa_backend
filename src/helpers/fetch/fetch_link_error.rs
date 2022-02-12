use std::fmt;

use crate::helpers::where_was::WhereWas;

#[derive(Debug, ImplDisplayDerive)]
pub struct FetchLinkError {
    pub source: Box<FetchLinkErrorEnum>,
}

#[derive(Debug)]
pub enum FetchLinkErrorEnum {
    ReqwestBlockingGet {
        source: reqwest::Error,
        where_was: WhereWas,
    },
    StatusCode {
        source: reqwest::StatusCode,
        where_was: WhereWas,
    },
    ResponseText {
        source: reqwest::Error,
        where_was: WhereWas,
    },
}
