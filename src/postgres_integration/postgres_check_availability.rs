use std::fmt;

use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(thiserror::Error, displaydoc::Display, Debug, BoxErrFromErrDerive)]
pub struct PostgresCheckAvailabilityError(
    /// postgres check availability error `{0}`
    #[source]
    Box<ConnectionError>,
);

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn postgres_check_availability(
    postgres_url: &str,
) -> Result<(), PostgresCheckAvailabilityError> {
    PgConnection::establish(postgres_url)?;
    Ok(())
}
