use diesel::prelude::ConnectionError;

use reqwest::StatusCode;

use thiserror::Error;

#[derive(Error, displaydoc::Display, Debug)]
pub enum CheckNetError {
    ///CheckNetError: StartingLinkCode: {0:?}
    StartingLinkCode(Box<StatusCode>),
    ///CheckNetError: reqwest: {0:?}
    ReqwestError(#[from] Box<reqwest::Error>),
    ///CheckNetError: Postgres: {0:?}
    Postgres(Box<ConnectionError>),
    ///CheckNetError: Mongo: {0:?}
    Mongo(#[from] Box<mongodb::error::Error>),
}
