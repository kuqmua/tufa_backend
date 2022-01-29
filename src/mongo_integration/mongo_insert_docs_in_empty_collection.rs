use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use crate::config_mods::lazy_static_config::CONFIG;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

#[derive(Debug)]
pub struct MongoInsertDocsInEmptyCollectionError {
    pub source: Box<MongoInsertDocsInEmptyCollectionErrorEnum>,
    line: String,
}

#[derive(Debug)]
pub enum MongoInsertDocsInEmptyCollectionErrorEnum {
    ClientOptionsParse(ClientOptionsParseError),
    ClientWithOptions(ClientWithOptionsError),
    CountDocuments(CountDocumentsError),
    NotEmpty(u64),
    CollectionInsertMany(CollectionInsertManyError),
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
pub struct CollectionInsertManyError {
    pub source: mongodb::error::Error,
    line: String,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_insert_docs_in_empty_collection(
    db_name_handle: &str,
    db_collection_handle: String,
    vec_of_values: Vec<String>,
) -> Result<(), MongoInsertDocsInEmptyCollectionError> {
    match ClientOptions::parse(mongo_get_db_url()).await {
        Err(e) => {
            return Err(MongoInsertDocsInEmptyCollectionError {
                source: Box::new(
                    MongoInsertDocsInEmptyCollectionErrorEnum::ClientOptionsParse(
                        ClientOptionsParseError {
                            source: e,
                            line: format!("{}:{}:{}", line!(), file!(), column!()),
                        },
                    ),
                ),
                line: format!("{}:{}:{}", line!(), file!(), column!()),
            })
        }
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => {
                return Err(MongoInsertDocsInEmptyCollectionError {
                    source: Box::new(
                        MongoInsertDocsInEmptyCollectionErrorEnum::ClientWithOptions(
                            ClientWithOptionsError {
                                source: e,
                                line: format!("{}:{}:{}", line!(), file!(), column!()),
                            },
                        ),
                    ),
                    line: format!("{}:{}:{}", line!(), file!(), column!()),
                })
            }
            Ok(client) => {
                let collection = client
                    .database(db_name_handle)
                    .collection(&db_collection_handle);
                match collection.count_documents(None, None).await {
                    Err(e) => {
                        return Err(MongoInsertDocsInEmptyCollectionError {
                            source: Box::new(
                                MongoInsertDocsInEmptyCollectionErrorEnum::CountDocuments(
                                    CountDocumentsError {
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
                        })
                    }
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            return Err(MongoInsertDocsInEmptyCollectionError {
                                source: Box::new(
                                    MongoInsertDocsInEmptyCollectionErrorEnum::NotEmpty(
                                        documents_number,
                                    ),
                                ),
                                line: format!("{}:{}:{}", line!(), file!(), column!()),
                            });
                        } else {
                            if let Err(e) = collection.insert_many(
                                vec_of_values.iter()
                                .map(|value|doc! { &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: value })
                                .collect::<Vec<Document>>(),
                                None).await {
                                return Err(MongoInsertDocsInEmptyCollectionError {
                                    source: Box::new(
                                        MongoInsertDocsInEmptyCollectionErrorEnum::CollectionInsertMany(
                                            CollectionInsertManyError { source: e, line: format!("{}:{}:{}", line!(), file!(), column!()), },
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
