use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoDropCollectionError {
    pub source: Box<MongoDropCollectionErrorEnum>,
    file: &'static str,
    line: u32,
    column: u32,
}

#[derive(Debug)]
pub enum MongoDropCollectionErrorEnum {
    ClientOptionsParse(ClientOptionsParseError),
    ClientWithOptions(ClientWithOptionsError),
    DatabaseDrop(DatabaseDropError),
}

#[derive(Debug)]
pub struct ClientOptionsParseError {
    pub source: mongodb::error::Error,
    file: &'static str,
    line: u32,
    column: u32,
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
    file: &'static str,
    line: u32,
    column: u32,
}

#[derive(Debug)]
pub struct DatabaseDropError {
    pub source: mongodb::error::Error,
    file: &'static str,
    line: u32,
    column: u32,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_collection(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), MongoDropCollectionError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => {
            return Err(MongoDropCollectionError {
                source: Box::new(MongoDropCollectionErrorEnum::ClientOptionsParse(
                    ClientOptionsParseError {
                        source: e,
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                )),
                file: file!(),
                line: line!(),
                column: column!(),
            })
        }
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => {
                return Err(MongoDropCollectionError {
                    source: Box::new(MongoDropCollectionErrorEnum::ClientWithOptions(
                        ClientWithOptionsError {
                            source: e,
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    )),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })
            }
            Ok(client) => {
                let collection: Collection<Document> =
                    client.database(db_name).collection(db_collection_name);
                if let Err(e) = collection.drop(None).await {
                    return Err(MongoDropCollectionError {
                        source: Box::new(MongoDropCollectionErrorEnum::DatabaseDrop(
                            DatabaseDropError {
                                source: e,
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        )),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    });
                }
                Ok(())
            }
        },
    }
}
