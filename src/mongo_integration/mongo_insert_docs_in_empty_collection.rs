use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use crate::config_mods::lazy_static_config::CONFIG;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[derive(Debug, Clone)]
pub enum MongoInsertDocsInEmptyCollection {
    MongoError(mongodb::error::Error),
    NotEmpty(u64),
}

impl From<mongodb::error::Error> for MongoInsertDocsInEmptyCollection {
    fn from(e: mongodb::error::Error) -> Self {
        MongoInsertDocsInEmptyCollection::MongoError(e)
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_insert_docs_in_empty_collection(
    db_name_handle: &str,
    db_collection_handle: String,
    vec_of_values: Vec<String>,
) -> Result<(), MongoInsertDocsInEmptyCollection> {
    let client_options = ClientOptions::parse(&mongo_get_db_url()).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(db_name_handle);
    let collection = db.collection(&db_collection_handle);
    let documents_number = collection.count_documents(None, None).await?;
    if documents_number > 0 {
        print_colorful_message(
            None,
            PrintType::WarningHigh,
            file!().to_string(),
            line!().to_string(),
            "collection is not empty, docs did not inserted".to_string(),
        );
        Err(MongoInsertDocsInEmptyCollection::NotEmpty(documents_number))
    } else {
        let mut docs: Vec<Document> = Vec::with_capacity(vec_of_values.len());
        for value in &vec_of_values {
            docs.push(doc! { &CONFIG.providers_db_collection_document_field_name_handle: value });
        }
        collection.insert_many(docs, None).await?;
        Ok(())
    }
}
