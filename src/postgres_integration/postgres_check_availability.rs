use std::time::Duration;

use sqlx::postgres::PgPoolOptions;

use crate::helpers::where_was::WhereWas;

#[derive(Debug)]
pub struct PostgresCheckAvailabilityError {
    pub source: sqlx::Error,
    pub where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_check_availability(
    postgres_url: &str,
) -> Result<(), Box<PostgresCheckAvailabilityError>> {
    if let Err(e) = PgPoolOptions::new()
        .max_connections(1)
        .connect_timeout(Duration::from_millis(1000))
        .connect(postgres_url)
        .await
    {
        return Err(Box::new(PostgresCheckAvailabilityError {
            source: e,
            where_was: WhereWas {
                file: file!(),
                line: line!(),
                column: column!(),
            },
        }));
    }
    Ok(())
}
