use mongodb::{options::ClientOptions, Client};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_db(mongo_url: &str, db_name: &str) -> Result<(), mongodb::error::Error> {
    Client::with_options(ClientOptions::parse(mongo_url).await?)?
        .database(db_name)
        .drop(None)
        .await?;
    Ok(())
}
