use crate::lazy_static::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use init_error::InitError;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use std::collections::HashMap;
use std::time::Duration;
use tufa_common::traits::with_tracing::WithTracing;
use tufa_common::where_was::WhereWas;

#[derive(Debug, InitError)] //, ImplGetWhereWasForErrorStruct
pub struct PostgresEstablishConnectionError {
    pub source: sqlx::Error,
    pub where_was: WhereWas,
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
    for PostgresEstablishConnectionError
{
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        tufa_common::where_was::WhereWasOneOrMany::One(
            tufa_common::where_was::WhereWasWithAddition {
                additional_info: None,
                where_was: self.where_was.clone(),
            },
        )
    }
}

impl tufa_common::traits::with_tracing::WithTracing<sqlx::Error>
    for PostgresEstablishConnectionError
{
    fn with_tracing(source: sqlx::Error, where_was: WhereWas) -> Self {
        match crate::lazy_static::config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = format!("{}", source),
                    source_place = where_was.file_line_column(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = format!("{}", source),
                    github_source_place = where_was
                        .github_file_line_column(&crate::lazy_static::git_info::GIT_INFO.data),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = format!("{}", source));
            }
        }
        Self { source, where_was }
    }
}

impl tufa_common::traits::get_source::GetSource for PostgresEstablishConnectionError {
    fn get_source(&self) -> String {
        match crate::lazy_static::config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self.source),
            false => format!("{}", self.source),
        }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_establish_connection(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    should_trace: bool,
) -> Result<sqlx::Pool<Postgres>, Box<PostgresEstablishConnectionError>> {
    match PgPoolOptions::new()
        .max_connections(providers_json_local_data_hashmap.len() as u32)
        .connect_timeout(Duration::from_millis(CONFIG.postgres_connection_timeout)) //todo add timeout constant or env var
        .connect(&CONFIG.get_postgres_url())
        .await
    {
        Err(e) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(PostgresEstablishConnectionError::with_tracing(
                    e, where_was,
                ))),
                false => Err(Box::new(PostgresEstablishConnectionError::new(
                    e, where_was,
                ))),
            }
        }
        Ok(pool) => Ok(pool),
    }
}
