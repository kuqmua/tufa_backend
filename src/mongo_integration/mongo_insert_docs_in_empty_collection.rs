use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use crate::config_mods::lazy_static_config::CONFIG;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[derive(Debug)]
pub struct MongoInsertDocsInEmptyCollectionError {
    pub source: Box<MongoInsertDocsInEmptyCollectionErrorEnum>
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
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
}

#[derive(Debug)]
pub struct CountDocumentsError {
    pub source: mongodb::error::Error,
}

#[derive(Debug)]
pub struct CollectionInsertManyError {
    pub source: mongodb::error::Error,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_insert_docs_in_empty_collection(
    db_name_handle: &str,
    db_collection_handle: String,
    vec_of_values: Vec<String>,
) -> Result<(), MongoInsertDocsInEmptyCollectionError> {
    //
    let client_options: ClientOptions;
    match ClientOptions::parse(mongo_get_db_url()).await {
        Err(e) => {
            return Err(MongoInsertDocsInEmptyCollectionError {
                source: Box::new(MongoInsertDocsInEmptyCollectionErrorEnum::ClientOptionsParse(
                    ClientOptionsParseError { source: e },
                )),
            })
        }
        Ok(cl) => client_options = cl,
    }
    let client: Client;
    match Client::with_options(client_options) {
        Err(e) => {
            return Err(MongoInsertDocsInEmptyCollectionError {
                source: Box::new(MongoInsertDocsInEmptyCollectionErrorEnum::ClientWithOptions(
                    ClientWithOptionsError { source: e },
                )),
            })
        }
        Ok(c) => client = c,
    }
    let db = client.database(db_name_handle);
    let collection = db.collection(&db_collection_handle);
    let documents_number: u64;
    match collection.count_documents(None, None).await {
        Err(e) => return Err(MongoInsertDocsInEmptyCollectionError {
            source: Box::new(MongoInsertDocsInEmptyCollectionErrorEnum::CountDocuments(
                CountDocumentsError { source: e },
            )),
        }),
        Ok(dn) => documents_number = dn,
    }
    if documents_number > 0 {
        print_colorful_message(
            None,
            PrintType::WarningHigh,
            file!().to_string(),
            line!().to_string(),
            "collection is not empty, docs did not inserted".to_string(),
        );
        Err(MongoInsertDocsInEmptyCollectionError {
            source: Box::new(
                MongoInsertDocsInEmptyCollectionErrorEnum::NotEmpty(documents_number)
            )
        }
        )
    } else {
        let mut docs: Vec<Document> = Vec::with_capacity(vec_of_values.len());
        for value in &vec_of_values {
            docs.push(doc! { &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: value });
        }
        if let Err(e) = collection.insert_many(docs, None).await {
            return Err(MongoInsertDocsInEmptyCollectionError {
                source: Box::new(MongoInsertDocsInEmptyCollectionErrorEnum::CollectionInsertMany(
                    CollectionInsertManyError { source: e },
                )),
            });
        }
        Ok(())
    }
}
