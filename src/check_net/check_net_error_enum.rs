use diesel::prelude::ConnectionError;

use reqwest::StatusCode;

#[derive(Debug)]
pub enum CheckNetError {
    StartingLinkCode(Box<StatusCode>),
    ReqwestError(Box<reqwest::Error>),
    Postgres(Box<ConnectionError>),
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
