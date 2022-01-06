use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_collection_is_empty(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<bool, mongodb::error::Error> {
    Ok(
        Client::with_options(ClientOptions::parse(mongo_url).await?)?
            .database(db_name) //declare db name. there is no create db method in mongo
            .collection::<Document>(db_collection_name)
            .count_documents(None, None)
            .await?
            == 0,
    )
}
