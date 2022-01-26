use std::time::Duration;

use sqlx::postgres::PgPoolOptions;

#[derive(Debug, BoxErrFromErrDerive)]
pub struct PostgresCheckAvailabilityError {
    pub source: Box<sqlx::Error>,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn postgres_check_availability(
    postgres_url: &str,
) -> Result<(), PostgresCheckAvailabilityError> {
    PgPoolOptions::new()
        .max_connections(1)
        .connect_timeout(Duration::from_millis(1000))
        .connect(postgres_url)
        .await?;
    Ok(())
}
