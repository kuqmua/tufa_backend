use mongodb::{options::ClientOptions, Client};

use crate::config_mods::lazy_static_config::CONFIG;

#[derive(Debug)]
pub struct MongoCheckAvailabilityError {
    pub source: Box<MongoCheckAvailabilityErrorEnum>,
}

#[derive(Debug)]
pub enum MongoCheckAvailabilityErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
        ClientWithOptions {
        source: mongodb::error::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    ListCollectionNames {
        source: mongodb::error::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_availability(mongo_url: &str) -> Result<(), MongoCheckAvailabilityError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(MongoCheckAvailabilityError {
            source: Box::new(MongoCheckAvailabilityErrorEnum::ClientOptionsParse {
                    source: e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            ),
        }),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(MongoCheckAvailabilityError {
                source: Box::new(MongoCheckAvailabilityErrorEnum::ClientWithOptions {
                        source: e,
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                ),
            }),
            Ok(client) => {
                if let Err(e) = client
                    .database(&CONFIG.mongo_providers_logs_db_name)
                    .list_collection_names(None)
                    .await
                {
                    return Err(MongoCheckAvailabilityError {
                        source: Box::new(MongoCheckAvailabilityErrorEnum::ListCollectionNames {
                                source: e,
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        ),
                    });
                }
                Ok(())
            }
        },
    }
}
