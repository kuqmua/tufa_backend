use std::fmt;

use sqlx::postgres::PgPoolOptions;
use sqlx::Error;

#[derive(thiserror::Error, displaydoc::Display, Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct PostgresCheckAvailabilityError {
    /// postgres check availability error `{0}`
    pub source: Box<Error>,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn postgres_check_availability(
    postgres_url: &str,
) -> Result<(), PostgresCheckAvailabilityError> {
    PgPoolOptions::new()
        .max_connections(1)
        .connect(postgres_url).await?;
    Ok(())
}
