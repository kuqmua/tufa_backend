use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasOneOrFew;
use crate::traits::tufa_error::TufaError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use init_error::DeriveInitError;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::fmt;
use std::time::Duration;
// use init_error_with_tracing::DeriveInitErrorWithTracing;
//DeriveInitErrorWithTracing,
#[derive(Debug, DeriveInitError)]
pub struct MongoCheckAvailabilityError {
    source: MongoCheckAvailabilityErrorEnum,
    where_was: Vec<WhereWasOneOrFew>,
}

impl MongoCheckAvailabilityError {
    pub fn with_tracing(
        source: MongoCheckAvailabilityErrorEnum,
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
                                    source = where_was_one.source_place(),
                                );
                            }
                            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                                tracing::error!(
                                    error = format!("{}", source),
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

impl TufaError for MongoCheckAvailabilityError {
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

impl fmt::Display for MongoCheckAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => {
                let content =
                    self.where_was
                        .clone()
                        .iter()
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&format!("{}\n", elem));
                            acc
                        });
                write!(f, "{}\n{}", self.source, content)
            }
        }
    }
}

#[derive(Debug)]
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
    should_trace: bool,
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
            match should_trace {
                true => Err(Box::new(MongoCheckAvailabilityError::with_tracing(
                    MongoCheckAvailabilityErrorEnum::ClientOptionsParse(e),
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
                false => Err(Box::new(MongoCheckAvailabilityError::new(
                    MongoCheckAvailabilityErrorEnum::ClientOptionsParse(e),
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
            }
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
                    match should_trace {
                        true => Err(Box::new(MongoCheckAvailabilityError::with_tracing(
                            MongoCheckAvailabilityErrorEnum::ClientWithOptions(e),
                            vec![WhereWasOneOrFew::One(where_was)],
                        ))),
                        false => Err(Box::new(MongoCheckAvailabilityError::new(
                            MongoCheckAvailabilityErrorEnum::ClientWithOptions(e),
                            vec![WhereWasOneOrFew::One(where_was)],
                        ))),
                    }
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
                        match should_trace {
                            true => {
                                return Err(Box::new(MongoCheckAvailabilityError::with_tracing(
                                    MongoCheckAvailabilityErrorEnum::ListCollectionNames(e),
                                    vec![WhereWasOneOrFew::One(where_was)],
                                )));
                            }
                            false => {
                                return Err(Box::new(MongoCheckAvailabilityError::new(
                                    MongoCheckAvailabilityErrorEnum::ListCollectionNames(e),
                                    vec![WhereWasOneOrFew::One(where_was)],
                                )));
                            }
                        }
                    }
                    Ok(())
                }
            }
        }
    }
}
