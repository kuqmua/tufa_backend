use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoCheckCollectionIsEmptyError {
    pub source: Box<MongoCheckCollectionIsEmptyErrorEnum>,
}

#[derive(Debug)]
pub enum MongoCheckCollectionIsEmptyErrorEnum {
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
    CountDocuments {
        source: mongodb::error::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    NotEmpty {
        source: u64,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_collection_is_empty(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), MongoCheckCollectionIsEmptyError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(MongoCheckCollectionIsEmptyError {
            source: Box::new(MongoCheckCollectionIsEmptyErrorEnum::ClientOptionsParse {
                source: e,
                file: file!(),
                line: line!(),
                column: column!(),
            }),
        }),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(MongoCheckCollectionIsEmptyError {
                source: Box::new(MongoCheckCollectionIsEmptyErrorEnum::ClientWithOptions {
                    source: e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                }),
            }),
            Ok(client) => {
                match client
                    .database(db_name)
                    .collection::<Document>(db_collection_name)
                    .count_documents(None, None)
                    .await
                {
                    Err(e) => Err(MongoCheckCollectionIsEmptyError {
                        source: Box::new(MongoCheckCollectionIsEmptyErrorEnum::CountDocuments {
                            source: e,
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        }),
                    }),
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            return Err(MongoCheckCollectionIsEmptyError {
                                source: Box::new(MongoCheckCollectionIsEmptyErrorEnum::NotEmpty {
                                    source: documents_number,
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                }),
                            });
                        }
                        Ok(())
                    }
                }
            }
        },
    }
}
