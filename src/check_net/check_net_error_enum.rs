use diesel::prelude::ConnectionError;

use reqwest::StatusCode;

use thiserror::Error;

#[derive(Error, displaydoc::Display, Debug)]
pub enum CheckNetError {
    ///starting link code
    StartingLinkCode(Box<StatusCode>),
    ///reqwest error
    ReqwestError(#[from] Box<reqwest::Error>),
    ///CheckNetError: postgres connection error: {0:?}"
    Postgres(Box<ConnectionError>),
    ///mongo
    Mongo(#[from] Box<mongodb::error::Error>),
}
