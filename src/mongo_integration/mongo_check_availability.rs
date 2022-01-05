use mongodb::{options::ClientOptions, Client};

use crate::config_mods::lazy_static_config::CONFIG;

use std::fmt;

use mongodb::error::Error;

#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub struct MongoCheckAvailabilityError {
    /// mongo check availability error `{0}`
    #[source]
    source: Box<Error>,
}

impl fmt::Display for MongoCheckAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl From<mongodb::error::Error> for MongoCheckAvailabilityError {
    fn from(error: mongodb::error::Error) -> Self {
        MongoCheckAvailabilityError {
            source: Box::new(error),
        }
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[tokio::main] //must be coz diesel(postgres) blocking inside tokio runtime - cause panic
pub async fn mongo_check_availability(mongo_url: &str) -> Result<(), MongoCheckAvailabilityError> {
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&CONFIG.mongo_providers_logs_db_name);
    db.list_collection_names(None).await?;
    Ok(())
}
