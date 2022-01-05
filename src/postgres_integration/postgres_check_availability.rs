use std::fmt;

use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub struct PostgresCheckAvailabilityError {
    /// postgres check availability error `{0}`
    pub source: Box<ConnectionError>,
}

impl fmt::Display for PostgresCheckAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ConnectionError> for PostgresCheckAvailabilityError {
    fn from(error: ConnectionError) -> Self {
        PostgresCheckAvailabilityError {
            source: Box::new(error),
        }
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn postgres_check_availability(
    postgres_url: &str,
) -> Result<(), PostgresCheckAvailabilityError> {
    PgConnection::establish(postgres_url)?;
    Ok(())
}
