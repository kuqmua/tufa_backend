use std::collections::HashMap;
use std::time::Duration;
use std::fmt;

use sqlx::postgres::PgPoolOptions;

use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::providers::provider_kind_enum::ProviderKind;

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct PostgresInitError {
    pub source: Box<PostgresInitErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum PostgresInitErrorEnum {
    // LoadingProvidersLinkParts(diesel::result::Error),
    // ProvidersLinkPartsIsNotEmpty(i64),
    // InsertPosts(diesel::result::Error),
    EstablishConnection(sqlx::Error),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_postgres(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), PostgresInitError> {
    let connection = PgPoolOptions::new()
    .max_connections(1)
    .connect_timeout(Duration::from_millis(1000))
    .connect(&postgres_get_db_url()).await?;
    // match todo_connection(&postgres_get_db_url()) {
    //     Err(e) => Err(PostgresInitError::TodoError(e)),
    //     Ok(pg_connection) => {
    //         // Todo: all tables initialization as one transaction?
    //         todo!()
    //     }
    // }
    todo!()
}   