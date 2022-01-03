use diesel::prelude::ConnectionError;

use reqwest::StatusCode;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckNetError {
    #[error("starting link code")]
    StartingLinkCode(Box<StatusCode>),
    #[error("reqwest error")]
    ReqwestError(Box<reqwest::Error>),
    #[error("postgres")]
    Postgres(Box<ConnectionError>),
    #[error("mongo")]
    Mongo(Box<mongodb::error::Error>),
}
// impl From<Box<mongodb::error::Error>> for CheckNetError {
//     fn from(e: Box<mongodb::error::Error>) -> Self {
//         CheckNetError::Mongo { error: e }
//     }
// }
// impl From<Box<ConnectionError>> for CheckNetError {
//     fn from(e: Box<ConnectionError>) -> Self {
//         CheckNetError::Postgres { error: e }
//     }
// }
// impl From<Box<reqwest::Error>> for CheckNetError {
//     fn from(e: Box<reqwest::Error>) -> Self {
//         CheckNetError::ReqwestError { error: e }
//     }
// }
