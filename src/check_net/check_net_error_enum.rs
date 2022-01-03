use diesel::prelude::ConnectionError;

use reqwest::StatusCode;

#[derive(Debug)]
pub enum CheckNetError {
    StartingLinkCode { status_code: StatusCode },
    ReqwestError { error: Box<reqwest::Error> },
    Postgres { error: ConnectionError },
    Mongo { error: mongodb::error::Error },
}
impl From<mongodb::error::Error> for CheckNetError {
    fn from(e: mongodb::error::Error) -> Self {
        CheckNetError::Mongo { error: e }
    }
}
impl From<ConnectionError> for CheckNetError {
    fn from(e: ConnectionError) -> Self {
        CheckNetError::Postgres { error: e }
    }
}
impl From<Box<reqwest::Error>> for CheckNetError {
    fn from(e: Box<reqwest::Error>) -> Self {
        CheckNetError::ReqwestError { error: e }
    }
}
