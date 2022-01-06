use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_collection(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), mongodb::error::Error> {
    Client::with_options(ClientOptions::parse(mongo_url).await?)?
        .database(db_name)
        .collection::<Document>(db_collection_name)
        .drop(None)
        .await?;
    Ok(())
}
