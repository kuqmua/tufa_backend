use diesel::prelude::ConnectionError;

use reqwest::StatusCode;

#[derive(Debug)]
pub enum CheckNetError {
    StartingLinkCode { status_code: StatusCode },
    StartingLinkDynError { error: Box<dyn std::error::Error> },
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
impl From<Box<dyn std::error::Error>> for CheckNetError {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        CheckNetError::StartingLinkDynError { error: e }
    }
}
