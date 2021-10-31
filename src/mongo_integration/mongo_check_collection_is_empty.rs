use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_collection_is_empty(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_handle: &str,
) -> Result<bool, mongodb::error::Error> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(db_name_handle); //declare db name. there is no create db method in mongo
    let documents_number = db
        .collection::<Document>(db_collection_handle)
        .count_documents(None, None)
        .await?;
    if documents_number == 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}
