use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub struct MongoCheckAvailabilityError {
    source: MongoCheckAvailabilityErrorEnum,
    where_was: Vec<WhereWas>,
}

impl MongoCheckAvailabilityError {
    pub fn new(source: MongoCheckAvailabilityErrorEnum, where_was: Vec<WhereWas>) -> Self {
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

impl fmt::Display for MongoCheckAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => write!(f, "{}\n{:?}", self.source, self.where_was),
        }
    }
}

#[derive(Debug)] //, ErrorDisplay
pub enum MongoCheckAvailabilityErrorEnum {
    ClientOptionsParse(mongodb::error::Error),
    ClientWithOptions(mongodb::error::Error),
    ListCollectionNames(mongodb::error::Error),
}

impl fmt::Display for MongoCheckAvailabilityErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => match self {
                MongoCheckAvailabilityErrorEnum::ClientOptionsParse(e) => write!(f, "{}", e),
                MongoCheckAvailabilityErrorEnum::ClientWithOptions(e) => write!(f, "{}", e),
                MongoCheckAvailabilityErrorEnum::ListCollectionNames(e) => write!(f, "{}", e),
            },
        }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_check_availability(
    mongo_url: &str,
) -> Result<(), Box<MongoCheckAvailabilityError>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            Err(Box::new(MongoCheckAvailabilityError::new(
                MongoCheckAvailabilityErrorEnum::ClientOptionsParse(e),
                vec![where_was],
            )))
        }
        Ok(mut client_options) => {
            client_options.connect_timeout =
                Some(Duration::from_millis(CONFIG.mongo_connection_timeout));
            match Client::with_options(client_options) {
                Err(e) => {
                    let where_was = WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    };
                    Err(Box::new(MongoCheckAvailabilityError::new(
                        MongoCheckAvailabilityErrorEnum::ClientWithOptions(e),
                        vec![where_was],
                    )))
                }
                Ok(client) => {
                    if let Err(e) = client
                        .database(&CONFIG.mongo_providers_logs_db_name)
                        .list_collection_names(None)
                        .await
                    {
                        let where_was = WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        };
                        return Err(Box::new(MongoCheckAvailabilityError::new(
                            MongoCheckAvailabilityErrorEnum::ListCollectionNames(e),
                            vec![where_was],
                        )));
                    }
                    Ok(())
                }
            }
        }
    }
}
