use mongodb::{options::ClientOptions, Client};

use crate::config_mods::lazy_static_config::CONFIG;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[tokio::main] //must be coz diesel(postgres) blocking inside tokio runtime - cause panic
pub async fn mongo_check_availability(mongo_url: &str) -> Result<(), Box<mongodb::error::Error>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(e)),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(Box::new(e)),
            Ok(client) => {
                let db = client.database(&CONFIG.mongo_providers_logs_db_name);
                if let Err(e) = db.list_collection_names(None).await {
                    return Err(Box::new(e));
                }
                Ok(())
            }
        },
    }
}
