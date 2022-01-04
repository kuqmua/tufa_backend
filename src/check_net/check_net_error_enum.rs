use diesel::prelude::ConnectionError;

use thiserror::Error;

use crate::check_net::check_net_availability::CheckNetAvailabilityError;

#[derive(Error, displaydoc::Display, Debug)]
pub enum CheckNetError {
    ///CheckNetError:  CheckNetAvailabilityError: {source:?}
    CheckNetAvailabilityError {
        source: Box<CheckNetAvailabilityError>,
    },
    ///CheckNetError: Postgres: {source:?}
    Postgres { source: Box<ConnectionError> },
    ///CheckNetError: Mongo: {source:?}
    Mongo { source: Box<mongodb::error::Error> },
}
