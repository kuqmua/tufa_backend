use crate::config_mods::lazy_static_config::CONFIG;

use crate::check_net::check_net_availability::check_net_availability;

use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityError;
use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::check_net::check_net_availability::NetCheckAvailabilityError;

use std::fmt;

#[derive(thiserror::Error, displaydoc::Display, Debug, ImplDisplayDerive)]
pub struct CheckNetWrapperError {
    /// check net wrapper error {source:?}
    #[source]
    pub source: Box<CheckNetWrapperErrorEnum>,
}

impl From<NetCheckAvailabilityError> for CheckNetWrapperError {
    fn from(error: NetCheckAvailabilityError) -> Self {
        CheckNetWrapperError {
            source: Box::new(CheckNetWrapperErrorEnum::NetCheckAvailabilityError(error)),
        }
    }
}

impl From<PostgresCheckAvailabilityError> for CheckNetWrapperError {
    fn from(error: PostgresCheckAvailabilityError) -> Self {
        CheckNetWrapperError {
            source: Box::new(CheckNetWrapperErrorEnum::PostgresCheckAvailabilityError(
                error,
            )),
        }
    }
}

impl From<MongoCheckAvailabilityError> for CheckNetWrapperError {
    fn from(error: MongoCheckAvailabilityError) -> Self {
        CheckNetWrapperError {
            source: Box::new(CheckNetWrapperErrorEnum::MongoCheckAvailabilityError(error)),
        }
    }
}

#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub enum CheckNetWrapperErrorEnum {
    /// net check availability error {0:?}
    NetCheckAvailabilityError(NetCheckAvailabilityError),
    /// postgres check availability error {0:?}
    PostgresCheckAvailabilityError(PostgresCheckAvailabilityError),
    /// mongo check availability error {0:?}
    MongoCheckAvailabilityError(MongoCheckAvailabilityError),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_wrapper() -> Result<(), CheckNetWrapperError> {
    //todo to it in parallel?
    check_net_availability(&CONFIG.starting_check_link)?;
    postgres_check_availability(&postgres_get_db_url())?;
    mongo_check_availability(&mongo_get_db_url())?;
    Ok(())
}
