use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasOneOrFew;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use init_error::DeriveInitError;
use init_error_with_tracing::DeriveInitErrorWithTracing;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use std::fmt;
use std::time::Duration;

#[derive(Debug, DeriveInitErrorWithTracing, DeriveInitError)]
pub struct PostgresCheckAvailabilityError {
    source: Error,
    where_was: Vec<WhereWasOneOrFew>,
}

impl fmt::Display for PostgresCheckAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => write!(f, "{}\n{:?}", self.source, self.where_was),
        }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_check_availability(
    postgres_url: &str,
    should_trace: bool,
) -> Result<(), Box<PostgresCheckAvailabilityError>> {
    if let Err(e) = PgPoolOptions::new()
        .max_connections(1)
        .connect_timeout(Duration::from_millis(CONFIG.postgres_connection_timeout))
        .connect(postgres_url)
        .await
    {
        let where_was = WhereWas {
            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file: file!(),
            line: line!(),
            column: column!(),
        };
        match should_trace {
            true => {
                return Err(Box::new(PostgresCheckAvailabilityError::with_tracing(
                    e,
                    vec![WhereWasOneOrFew::One(where_was)],
                )));
            }
            false => {
                return Err(Box::new(PostgresCheckAvailabilityError::new(
                    e,
                    vec![WhereWasOneOrFew::One(where_was)],
                )));
            }
        }
    }
    Ok(())
}
