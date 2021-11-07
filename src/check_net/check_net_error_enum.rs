use crate::check_net::fetch_link::fetch_link;

use crate::config_mods::config::CONFIG;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

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
