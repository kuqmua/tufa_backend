use mongodb::{options::ClientOptions, Client};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_availability(mongo_url: &str) -> Result<(), mongodb::error::Error> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    Client::with_options(client_options)?;
    Ok(())
}
