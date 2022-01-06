use mongodb::error::Error;
use mongodb::{options::ClientOptions, Client};

use std::fmt;

use crate::config_mods::lazy_static_config::CONFIG;

#[derive(thiserror::Error, displaydoc::Display, Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct MongoCheckAvailabilityError {
    /// mongo check availability error `{0}`
    #[source]
    source: Box<Error>,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_check_availability(mongo_url: &str) -> Result<(), MongoCheckAvailabilityError> {
    Client::with_options(ClientOptions::parse(mongo_url).await?)?
        .database(&CONFIG.mongo_providers_logs_db_name)
        .list_collection_names(None)
        .await?;
    Ok(())
}
