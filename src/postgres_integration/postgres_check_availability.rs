use std::fmt;
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

impl fmt::Display for PostgresCheckAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled {
            write!(
                f,
                "{}\n{}\n{}",
                self.where_was.source_place_with_readable_time(),
                self.where_was.github_source_place_with_readable_time(),
                self.source
            )
        } else if CONFIG.is_show_source_place_enabled {
            write!(
                f,
                "{}\n{}",
                self.where_was.source_place_with_readable_time(),
                self.source
            )
        } else if CONFIG.is_show_github_source_place_enabled {
            write!(
                f,
                "{}\n{}",
                self.where_was.github_source_place_with_readable_time(),
                self.source
            )
        } else {
            write!(f, "{}", self.source)
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
