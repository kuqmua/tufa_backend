use mongodb::{options::ClientOptions, Client};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_db_is_empty(
    mongo_url: &str,
    db_name: &str,
) -> Result<bool, mongodb::error::Error> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(db_name);
    let collections_names_list = db.list_collection_names(None).await?;
    Ok(collections_names_list.is_empty())
}
