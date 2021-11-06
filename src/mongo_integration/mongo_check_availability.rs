use mongodb::{options::ClientOptions, Client};

use crate::config_mods::config::CONFIG;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[tokio::main] //must be coz diesel(postgres) blocking inside tokio runtime - cause panic
pub async fn mongo_check_availability(mongo_url: &str) -> Result<(), mongodb::error::Error> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&CONFIG.mongo_params.providers_db_name_handle);
    db.list_collection_names(None).await?; //todo: remove this and find something lite like ping
    Ok(())
}
