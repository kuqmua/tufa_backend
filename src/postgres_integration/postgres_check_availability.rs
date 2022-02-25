use std::time::Duration;

use chrono::{DateTime, FixedOffset, Local, Utc};
use sqlx::postgres::PgPoolOptions;

use crate::helpers::where_was::WhereWas;

use crate::config_mods::lazy_static_config::CONFIG;

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
        .connect_timeout(Duration::from_millis(CONFIG.postgres_connection_timeout))
        .connect(postgres_url)
        .await
    {
        return Err(Box::new(PostgresCheckAvailabilityError {
            source: e,
            where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(3 * 3600)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        }));
    }
    Ok(())
}
