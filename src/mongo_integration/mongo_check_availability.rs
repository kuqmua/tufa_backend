use std::fmt;
use std::time::Duration;

use chrono::{DateTime, FixedOffset, Local, Utc};

use mongodb::{options::ClientOptions, Client};

use crate::config_mods::lazy_static_config::CONFIG;

use crate::helpers::where_was::WhereWas;

#[derive(Debug)]
pub enum MongoCheckAvailabilityErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ListCollectionNames {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
}

impl fmt::Display for MongoCheckAvailabilityErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MongoCheckAvailabilityErrorEnum::ClientOptionsParse { source, where_was } => {
                if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled
                {
                    write!(
                        f,
                        "{}\n{}\n{}",
                        where_was.source_place_with_readable_time(),
                        where_was.github_source_place_with_readable_time(),
                        source
                    )
                } else if CONFIG.is_show_source_place_enabled {
                    write!(
                        f,
                        "{}\n{}",
                        where_was.source_place_with_readable_time(),
                        source
                    )
                } else if CONFIG.is_show_github_source_place_enabled {
                    write!(
                        f,
                        "{}\n{}",
                        where_was.github_source_place_with_readable_time(),
                        source
                    )
                } else {
                    write!(f, "{}", source)
                }
            }
            MongoCheckAvailabilityErrorEnum::ClientWithOptions { source, where_was } => {
                if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled
                {
                    write!(
                        f,
                        "{}\n{}\n{}",
                        where_was.source_place_with_readable_time(),
                        where_was.github_source_place_with_readable_time(),
                        source
                    )
                } else if CONFIG.is_show_source_place_enabled {
                    write!(
                        f,
                        "{}\n{}",
                        where_was.source_place_with_readable_time(),
                        source
                    )
                } else if CONFIG.is_show_github_source_place_enabled {
                    write!(
                        f,
                        "{}\n{}",
                        where_was.github_source_place_with_readable_time(),
                        source
                    )
                } else {
                    write!(f, "{}", source)
                }
            }
            MongoCheckAvailabilityErrorEnum::ListCollectionNames { source, where_was } => {
                if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled
                {
                    write!(
                        f,
                        "{}\n{}\n{}",
                        where_was.source_place_with_readable_time(),
                        where_was.github_source_place_with_readable_time(),
                        source
                    )
                } else if CONFIG.is_show_source_place_enabled {
                    write!(
                        f,
                        "{}\n{}",
                        where_was.source_place_with_readable_time(),
                        source
                    )
                } else if CONFIG.is_show_github_source_place_enabled {
                    write!(
                        f,
                        "{}\n{}",
                        where_was.github_source_place_with_readable_time(),
                        source
                    )
                } else {
                    write!(f, "{}", source)
                }
            }
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
) -> Result<(), Box<MongoCheckAvailabilityErrorEnum>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            MongoCheckAvailabilityErrorEnum::ClientOptionsParse {
                source: e,
                where_was: WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(3 * 3600)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            },
        )),
        Ok(mut client_options) => {
            client_options.connect_timeout =
                Some(Duration::from_millis(CONFIG.mongo_connection_timeout));
            match Client::with_options(client_options) {
                Err(e) => Err(Box::new(
                    MongoCheckAvailabilityErrorEnum::ClientWithOptions {
                        source: e,
                        where_was: WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(3 * 3600)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    },
                )),
                Ok(client) => {
                    if let Err(e) = client
                        .database(&CONFIG.mongo_providers_logs_db_name)
                        .list_collection_names(None)
                        .await
                    {
                        return Err(Box::new(
                            MongoCheckAvailabilityErrorEnum::ListCollectionNames {
                                source: e,
                                where_was: WhereWas {
                                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                        .with_timezone(&FixedOffset::east(3 * 3600)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            },
                        ));
                    }
                    Ok(())
                }
            }
        }
    }
}
