use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoDropEmptyCollectionError {
    pub source: Box<MongoDropEmptyCollectionErrorEnum>,
}

#[derive(Debug)]
pub enum MongoDropEmptyCollectionErrorEnum {
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
    DatabaseDrop {
        source: mongodb::error::Error,
        file: &'static str,
        line: u32,
        column: u32,
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_empty_collection(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), MongoDropEmptyCollectionError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(MongoDropEmptyCollectionError {
            source: Box::new(MongoDropEmptyCollectionErrorEnum::ClientOptionsParse {
                    source: e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            ),
        }),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(MongoDropEmptyCollectionError {
                source: Box::new(MongoDropEmptyCollectionErrorEnum::ClientWithOptions {
                        source: e,
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                ),
            }),
            Ok(client) => {
                let collection: Collection<Document> =
                    client.database(db_name).collection(db_collection_name);
                match collection.count_documents(None, None).await {
                    Err(e) => Err(MongoDropEmptyCollectionError {
                        source: Box::new(MongoDropEmptyCollectionErrorEnum::CountDocuments {
                                source: e,
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        ),
                    }),
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            Err(MongoDropEmptyCollectionError {
                                source: Box::new(MongoDropEmptyCollectionErrorEnum::NotEmpty {
                                    source: documents_number,
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                }),
                            })
                        } else {
                            if let Err(e) = collection.drop(None).await {
                                return Err(MongoDropEmptyCollectionError {
                                    source: Box::new(
                                        MongoDropEmptyCollectionErrorEnum::DatabaseDrop {
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
                    }
                }
            }
        },
    }
}
