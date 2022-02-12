use mongodb::{options::ClientOptions, Client};

use crate::helpers::where_was::WhereWas;

#[derive(Debug)]
pub struct MongoDropDbError {
    pub source: Box<MongoDropDbErrorEnum>,
}

#[derive(Debug)]
pub enum MongoDropDbErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    DatabaseDrop {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_db(mongo_url: &str, db_name: &str) -> Result<(), MongoDropDbError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(MongoDropDbError {
            source: Box::new(MongoDropDbErrorEnum::ClientOptionsParse {
                source: e,
                where_was: WhereWas {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            }),
        }),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(MongoDropDbError {
                source: Box::new(MongoDropDbErrorEnum::ClientWithOptions {
                    source: e,
                    where_was: WhereWas {
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                }),
            }),
            Ok(client) => {
                if let Err(e) = client.database(db_name).drop(None).await {
                    return Err(MongoDropDbError {
                        source: Box::new(MongoDropDbErrorEnum::DatabaseDrop {
                            source: e,
                            where_was: WhereWas {
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        }),
                    });
                }
                Ok(())
            }
        },
    }
}
