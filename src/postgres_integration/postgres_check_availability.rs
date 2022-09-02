use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_get_source_for_original_error_struct::ImplGetSourceForOriginalErrorStruct;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use std::time::Duration;

#[derive(
    Debug,
    ImplDisplayForErrorStruct,
    ImplGetSourceForOriginalErrorStruct,
    ImplGetWhereWasForErrorStruct,
    InitError,
)]
pub struct PostgresCheckAvailabilityError {
    source: Error,
    where_was: WhereWas,
}

impl PostgresCheckAvailabilityError {
    pub fn with_tracing(
        source: sqlx::Error,
        where_was: crate::helpers::where_was::WhereWas,
    ) -> Self {
        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = format!("{}", source),
                    source = where_was.source_place(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = format!("{}", source),
                    github_source = where_was.github_source_place(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = format!("{}", source));
            }
        }
        Self { source, where_was }
    }
    // pub fn new(source: sqlx::Error, where_was: WhereWas) -> Self {
    //     Self { source, where_was }
    // }
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
                    e, where_was,
                )));
            }
            false => {
                return Err(Box::new(PostgresCheckAvailabilityError::new(e, where_was)));
            }
        }
    }
    Ok(())
}
