use mongodb::bson::Document;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct MongoDropEmptyCollectionError {
    pub source: Box<MongoDropEmptyCollectionErrorEnum>,
    line: String,
}

#[derive(Debug)]
pub enum MongoDropEmptyCollectionErrorEnum {
    ClientOptionsParse(ClientOptionsParseError),
    ClientWithOptions(ClientWithOptionsError),
    CountDocuments(CountDocumentsError),
    NotEmpty(u64),
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
pub struct CountDocumentsError {
    pub source: mongodb::error::Error,
    line: String,
}

#[derive(Debug)]
pub struct DatabaseDropError {
    pub source: mongodb::error::Error,
    line: String,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_empty_collection(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), MongoDropEmptyCollectionError> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => {
            return Err(MongoDropEmptyCollectionError {
                source: Box::new(MongoDropEmptyCollectionErrorEnum::ClientOptionsParse(
                    ClientOptionsParseError {
                        source: e,
                        line: format!("{}:{}:{}", line!(), file!(), column!()),
                    },
                )),
                line: format!("{}:{}:{}", line!(), file!(), column!()),
            })
        }
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => {
                return Err(MongoDropEmptyCollectionError {
                    source: Box::new(MongoDropEmptyCollectionErrorEnum::ClientWithOptions(
                        ClientWithOptionsError {
                            source: e,
                            line: format!("{}:{}:{}", line!(), file!(), column!()),
                        },
                    )),
                    line: format!("{}:{}:{}", line!(), file!(), column!()),
                })
            }
            Ok(client) => {
                let collection: Collection<Document> =
                    client.database(db_name).collection(&db_collection_name);
                match collection.count_documents(None, None).await {
                    Err(e) => {
                        return Err(MongoDropEmptyCollectionError {
                            source: Box::new(MongoDropEmptyCollectionErrorEnum::CountDocuments(
                                CountDocumentsError {
                                    source: e,
                                    line: format!("{}:{}:{}", line!(), file!(), column!()),
                                },
                            )),
                            line: format!("{}:{}:{}", line!(), file!(), column!()),
                        })
                    }
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            return Err(MongoDropEmptyCollectionError {
                                source: Box::new(MongoDropEmptyCollectionErrorEnum::NotEmpty(
                                    documents_number,
                                )),
                                line: format!("{}:{}:{}", line!(), file!(), column!()),
                            });
                        } else {
                            if let Err(e) = collection.drop(None).await {
                                return Err(MongoDropEmptyCollectionError {
                                    source: Box::new(
                                        MongoDropEmptyCollectionErrorEnum::DatabaseDrop(
                                            DatabaseDropError {
                                                source: e,
                                                line: format!(
                                                    "{} {}",
                                                    line!().to_string(),
                                                    file!().to_string()
                                                ),
                                            },
                                        ),
                                    ),
                                    line: format!("{}:{}:{}", line!(), file!(), column!()),
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
