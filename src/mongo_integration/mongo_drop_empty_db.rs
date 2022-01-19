use std::fmt;

use mongodb::error::Error;
use mongodb::{options::ClientOptions, Client};

#[derive(thiserror::Error, displaydoc::Display, Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct MongoDropEmptyDbError {
    /// mongo drop empty database error `{0}`
    #[source]
    source: Box<Error>,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_drop_empty_db(
    mongo_url: &str,
    db_name: &str,
) -> Result<bool, MongoDropEmptyDbError> {
    let db = Client::with_options(ClientOptions::parse(mongo_url).await?)?.database(db_name);
    let collections_names_list = db.list_collection_names(None).await?;
    if !collections_names_list.is_empty() {
        return Ok(false);
    }
    db.drop(None).await?;
    Ok(true)
}
