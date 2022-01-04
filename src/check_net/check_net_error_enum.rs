use diesel::prelude::ConnectionError;

use reqwest::StatusCode;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckNetError {
    #[error("starting link code")]
    StartingLinkCode(Box<StatusCode>),
    #[error("reqwest error")]
    ReqwestError(#[from] Box<reqwest::Error>),
    #[error("CheckNetError: postgres connection error: {0:?}")]
    Postgres(Box<ConnectionError>),
    #[error("mongo")]
    Mongo(#[from] Box<mongodb::error::Error>),
}
