use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_collection(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), mongodb::error::Error> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(db_name);
    let collection = db.collection::<Document>(db_collection_name);
    collection.drop(None).await?;
    Ok(())
}
