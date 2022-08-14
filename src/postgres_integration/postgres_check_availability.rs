use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasTracing;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use sqlx::postgres::PgPoolOptions;
use std::fmt;
use std::time::Duration;
// use error_display::ErrorDisplay;

#[derive(Debug)] //, ErrorDisplay
pub struct PostgresCheckAvailabilityError {
    pub source: sqlx::Error,
    pub where_was: WhereWas,
}

impl fmt::Display for PostgresCheckAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => write!(f, "{}\n{}", self.source, self.where_was),
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
) -> Result<(), Box<PostgresCheckAvailabilityError>> {
    if let Err(e) = PgPoolOptions::new()
        .max_connections(1)
        .connect_timeout(Duration::from_millis(CONFIG.postgres_connection_timeout))
        .connect(postgres_url)
        .await
    {
        let where_was = WhereWas::new(
            DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file!(),
            line!(),
            column!(),
            Some(WhereWasTracing::Message(format!("{}", e))),
        );
        return Err(Box::new(PostgresCheckAvailabilityError {
            source: e,
            where_was,
        }));
    }
    Ok(())
}
