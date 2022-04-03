use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use chrono::{DateTime, FixedOffset, Local, Utc};
use mongodb::{options::ClientOptions, Client};
use std::fmt;
use std::time::Duration;

#[derive(Debug, ErrorDisplay)]
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
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
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
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
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
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
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
