use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

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
    let collection = Client::with_options(ClientOptions::parse(mongo_url).await?)?
        .database(db_name)
        .collection::<Document>(db_collection_name);
    let documents_number = collection.count_documents(None, None).await?;
    if documents_number > 0 {
        Err(MongoDropEmptyCollectionError::NotEmpty(documents_number))
    } else {
        collection.drop(None).await?;
        Ok(())
    }
}
