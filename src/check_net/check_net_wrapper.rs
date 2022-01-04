use crate::config_mods::lazy_static_config::CONFIG;

use crate::check_net::check_net_availability::check_net_availability;

use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::check_net::check_net_availability::CheckNetAvailabilityError;

#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub enum CheckNetError {
    ///CheckNetError:  CheckNetAvailabilityError: {0:?}
    CheckNetAvailabilityError(#[source] Box<CheckNetAvailabilityError>),
    ///CheckNetError: Postgres: {0:?}
    Postgres(#[source] PostgresCheckAvailabilityError),
    ///CheckNetError: Mongo: {0:?}
    Mongo(#[source] Box<mongodb::error::Error>),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_wrapper() -> Result<(), Box<CheckNetError>> {
    //todo to it in parallel?
    if let Err(e) = check_net_availability(&CONFIG.starting_check_link) {
        return Err(Box::new(CheckNetError::CheckNetAvailabilityError(e)));
    }
    if let Err(e) = postgres_check_availability(&postgres_get_db_url()) {
        return Err(Box::new(CheckNetError::Postgres(e)));
    }
    if let Err(e) = mongo_check_availability(&mongo_get_db_url()) {
        return Err(Box::new(CheckNetError::Mongo(e)));
    }
    Ok(())
}
