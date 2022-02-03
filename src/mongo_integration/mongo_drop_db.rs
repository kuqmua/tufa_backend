use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoDropDbError {
    pub source: Box<MongoDropDbErrorEnum>,
}

#[derive(Debug)]
pub enum MongoDropDbErrorEnum {
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
    DatabaseDrop {
        source: mongodb::error::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_db(mongo_url: &str, db_name: &str) -> Result<(), MongoDropDbError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(MongoDropDbError {
            source: Box::new(MongoDropDbErrorEnum::ClientOptionsParse {
                source: e,
                file: file!(),
                line: line!(),
                column: column!(),
            }),
        }),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(MongoDropDbError {
                source: Box::new(MongoDropDbErrorEnum::ClientWithOptions {
                    source: e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                }),
            }),
            Ok(client) => {
                if let Err(e) = client.database(db_name).drop(None).await {
                    return Err(MongoDropDbError {
                        source: Box::new(MongoDropDbErrorEnum::DatabaseDrop {
                            source: e,
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        }),
                    });
                }
                Ok(())
            }
        },
    }
}
