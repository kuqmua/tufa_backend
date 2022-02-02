use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use crate::config_mods::lazy_static_config::CONFIG;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

#[derive(Debug)]
pub enum MongoInsertDocsInEmptyCollectionErrorEnum {
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
    CollectionInsertMany {
        source: mongodb::error::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_insert_docs_in_empty_collection(
    db_name_handle: &str,
    db_collection_handle: String,
    vec_of_values: Vec<String>,
) -> Result<(), Box<MongoInsertDocsInEmptyCollectionErrorEnum>> {
    match ClientOptions::parse(mongo_get_db_url()).await {
        Err(e) => Err(Box::new(
            MongoInsertDocsInEmptyCollectionErrorEnum::ClientOptionsParse {
                source: e,
                file: file!(),
                line: line!(),
                column: column!(),
            },
        )),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(Box::new(
                MongoInsertDocsInEmptyCollectionErrorEnum::ClientWithOptions {
                    source: e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            )),
            Ok(client) => {
                let collection = client
                    .database(db_name_handle)
                    .collection(&db_collection_handle);
                match collection.count_documents(None, None).await {
                    Err(e) => Err(Box::new(
                        MongoInsertDocsInEmptyCollectionErrorEnum::CountDocuments {
                            source: e,
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    )),
                    Ok(documents_number) => {
                        if documents_number > 0 {
                            Err(Box::new(
                                MongoInsertDocsInEmptyCollectionErrorEnum::NotEmpty {
                                    source: documents_number,
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            ))
                        } else {
                            if let Err(e) = collection.insert_many(
                                vec_of_values.iter()
                                .map(|value|doc! { &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: value })
                                .collect::<Vec<Document>>(),
                                None).await {
                                return Err(
                                    Box::new(MongoInsertDocsInEmptyCollectionErrorEnum::CollectionInsertMany {
                                source: e,
                                            file: file!(),
            line: line!(),
            column: column!(),
                            }),
                            );
                            }
                            Ok(())
                        }
                    }
                }
            }
        },
    }
}
