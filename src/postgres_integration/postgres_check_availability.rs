use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasOneOrFew;
use crate::traits::tufa_error::TufaError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use init_error::DeriveInitError;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use std::fmt;
use std::time::Duration;

#[derive(Debug, DeriveInitError)]
pub struct PostgresCheckAvailabilityError {
    source: Error,
    where_was: Vec<WhereWasOneOrFew>,
}

impl TufaError for PostgresCheckAvailabilityError {
    fn get_source(&self) -> String {
        format!("{}", self.source)
    }
    fn get_where_was(&self) -> String {
        match CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self.where_was),
            false => {
                let mut content =
                    self.where_was
                        .clone()
                        .iter()
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&format!("{},", elem));
                            acc
                        });
                if !content.is_empty() {
                    content.pop();
                }
                content
            }
        }
    }
}

impl PostgresCheckAvailabilityError {
    pub fn with_tracing(
        source: Error,
        where_was: Vec<crate::helpers::where_was::WhereWasOneOrFew>,
    ) -> Self {
        if where_was.len() == 1 {
            if let Some(first_value) = where_was.get(0) {
                match first_value {
                    crate::helpers::where_was::WhereWasOneOrFew::One(where_was_one) => {
                        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
                            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                                tracing::error!(
                                    error = format!("{}", source),
                                    // children_source = format!("{}", &self.get_where_was()),
                                    source = where_was_one.source_place(),
                                );
                            }
                            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                                tracing::error!(
                                    error = format!("{}", source),
                                    // error = format!("{}", &self.get_source()),
                                    // children_source = format!("{}", &self.get_where_was()),
                                    github_source = where_was_one.github_source_place(),
                                );
                            }
                            crate::config_mods::source_place_type::SourcePlaceType::None => {
                                tracing::error!(error = format!("{}", source));
                            }
                        }
                    }
                    crate::helpers::where_was::WhereWasOneOrFew::Few(hs_where_was) => {
                        tracing::error!(error = "todo WhereWasOneOrFew::Few",)
                    }
                }
            }
            //todo next elements
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
