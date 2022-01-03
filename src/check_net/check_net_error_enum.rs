use diesel::prelude::ConnectionError;

use reqwest::StatusCode;

#[derive(Debug)]
pub enum CheckNetError {
    StartingLinkCode { status_code: Box<StatusCode> },
    ReqwestError { error: Box<reqwest::Error> },
    Postgres { error: Box<ConnectionError> },
    Mongo { error: Box<mongodb::error::Error> },
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
