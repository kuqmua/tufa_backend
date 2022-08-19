use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use sqlx::postgres::PgPoolOptions;
use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub struct PostgresCheckAvailabilityError {
    source: sqlx::Error,
    where_was: Vec<WhereWas>,
}

impl PostgresCheckAvailabilityError {
    pub fn new(source: sqlx::Error, where_was: Vec<WhereWas>) -> Self {
        if where_was.len() == 1 {
            if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled {
                tracing::error!(
                    error = format!("{}", source),
                    source = where_was[0].source_place(),
                    github_source = where_was[0].github_source_place(),
                );
            } else if CONFIG.is_show_source_place_enabled {
                tracing::error!(
                    error = format!("{}", source),
                    source = where_was[0].source_place(),
                );
            } else if CONFIG.is_show_github_source_place_enabled {
                tracing::error!(
                    error = format!("{}", source),
                    github_source = where_was[0].github_source_place(),
                );
            } else {
                tracing::error!(error = format!("{}", source),);
            }
        }
        Self { source, where_was }
    }
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
        return Err(Box::new(PostgresCheckAvailabilityError::new(
            e,
            vec![where_was],
        )));
    }
    Ok(())
}
