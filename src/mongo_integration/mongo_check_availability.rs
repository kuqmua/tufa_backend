use mongodb::{options::ClientOptions, Client};

use crate::config_mods::lazy_static_config::CONFIG;

#[derive(Debug)]
pub struct MongoCheckAvailabilityError {
    source: Box<MongoCheckAvailabilityErrorEnum>,
    line: String,
}

#[derive(Debug)]
pub enum MongoCheckAvailabilityErrorEnum {
    ClientOptionsParse(ClientOptionsParseError),
    ClientWithOptions(ClientWithOptionsError),
    ListCollectionNames(ListCollectionNamesError),
}

#[derive(Debug)]
pub struct ClientOptionsParseError {
    pub source: mongodb::error::Error,
    line: String,
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
    line: String,
}

#[derive(Debug)]
pub struct ListCollectionNamesError {
    pub source: mongodb::error::Error,
    line: String,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_availability(mongo_url: &str) -> Result<(), MongoCheckAvailabilityError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => {
            return Err(MongoCheckAvailabilityError {
                source: Box::new(MongoCheckAvailabilityErrorEnum::ClientOptionsParse(
                    ClientOptionsParseError {
                        source: e,
                        line: format!("{}:{}:{}", file!(), line!(), column!()),
                    },
                )),
                line: format!("{}:{}:{}", file!(), line!(), column!()),
            });
        }
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => {
                return Err(MongoCheckAvailabilityError {
                    source: Box::new(MongoCheckAvailabilityErrorEnum::ClientWithOptions(
                        ClientWithOptionsError {
                            source: e,
                            line: format!("{}:{}:{}", file!(), line!(), column!()),
                        },
                    )),
                    line: format!("{}:{}:{}", file!(), line!(), column!()),
                });
            }
            Ok(client) => {
                if let Err(e) = client
                    .database(&CONFIG.mongo_providers_logs_db_name)
                    .list_collection_names(None)
                    .await
                {
                    return Err(MongoCheckAvailabilityError {
                        source: Box::new(MongoCheckAvailabilityErrorEnum::ListCollectionNames(
                            ListCollectionNamesError {
                                source: e,
                                line: format!("{}:{}:{}", file!(), line!(), column!()),
                            },
                        )),
                        line: format!("{}:{}:{}", file!(), line!(), column!()),
                    });
                }
                Ok(())
            }
        },
    }
}
