use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoDropCollectionError {
    pub source: Box<MongoDropCollectionErrorEnum>,
    line: String,
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
    line: String,
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
    line: String,
}

#[derive(Debug)]
pub struct DatabaseDropError {
    pub source: mongodb::error::Error,
    line: String,
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
                        line: format!("{}:{}:{}", file!(), line!(), column!()),
                    },
                )),
                line: format!("{}:{}:{}", file!(), line!(), column!()),
            })
        }
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => {
                return Err(MongoDropCollectionError {
                    source: Box::new(MongoDropCollectionErrorEnum::ClientWithOptions(
                        ClientWithOptionsError {
                            source: e,
                            line: format!("{}:{}:{}", file!(), line!(), column!()),
                        },
                    )),
                    line: format!("{}:{}:{}", file!(), line!(), column!()),
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
