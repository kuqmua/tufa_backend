use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[derive(Debug)]
pub enum MongoDropEmptyCollectionError {
    MongoError(mongodb::error::Error),
    NotEmpty(u64),
}
impl From<mongodb::error::Error> for MongoDropEmptyCollectionError {
    fn from(e: mongodb::error::Error) -> Self {
        MongoDropEmptyCollectionError::MongoError(e)
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_empty_collection(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), MongoDropEmptyCollectionError> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let collection = client
        .database(db_name)
        .collection::<Document>(db_collection_name);
    let documents_number = collection.count_documents(None, None).await?;
    if documents_number > 0 {
        print_colorful_message(
            None,
            PrintType::WarningLow,
            file!().to_string(),
            line!().to_string(),
            "collection is not empty".to_string(),
        );
        Err(MongoDropEmptyCollectionError::NotEmpty(documents_number))
    } else {
        collection.drop(None).await?;
        Ok(())
    }
}
