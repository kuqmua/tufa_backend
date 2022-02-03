use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoDropCollectionError {
    pub source: Box<MongoDropCollectionErrorEnum>,
}

#[derive(Debug)]
pub enum MongoDropCollectionErrorEnum {
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
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_collection(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), MongoDropCollectionError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(MongoDropCollectionError {
            source: Box::new(MongoDropCollectionErrorEnum::ClientOptionsParse {
                    source: e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            ),
        }),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(MongoDropCollectionError {
                source: Box::new(MongoDropCollectionErrorEnum::ClientWithOptions {
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
                if let Err(e) = collection.drop(None).await {
                    return Err(MongoDropCollectionError {
                        source: Box::new(MongoDropCollectionErrorEnum::DatabaseDrop {
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
