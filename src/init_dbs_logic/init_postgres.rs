use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use sqlx::postgres::PgPoolOptions;

use crate::postgres_integration::postgres_delete_all_from_providers_tables::postgres_delete_all_from_providers_tables;
use crate::postgres_integration::postgres_delete_all_from_providers_tables::PostgresDeleteAllFromProvidersTablesError;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::postgres_insert_link_parts_into_providers_tables;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesError;
use crate::providers::provider_kind_enum::ProviderKind;

use crate::postgres_integration::postgres_check_provider_links_tables_are_empty::postgres_check_provider_links_tables_are_empty;
use crate::postgres_integration::postgres_check_provider_links_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyError;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::postgres_create_providers_tables_if_not_exists;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsError;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct PostgresInitError {
    pub source: Box<PostgresInitErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum PostgresInitErrorEnum {
    DeleteAllFromProvidersTables(PostgresDeleteAllFromProvidersTablesError),
    CheckProviderLinksTablesAreEmpty(PostgresCheckProvidersLinkPartsTablesEmptyError),
    CreateTableQueries(PostgresCreateProvidersDbsError),
    InsertLinkPartsIntoProvidersTables(PostgresInsertLinkPartsIntoProvidersTablesError),
    EstablishConnection(sqlx::Error),
}

pub type CreateTableQueriesHashmap = HashMap<ProviderKind, sqlx::Error>; //for wroking logic for now. todo: move into different function
pub type InsertQueriesHashmap = HashMap<ProviderKind, sqlx::Error>; //for wroking logic for now. todo: move into different function
#[deny(clippy::indexing_slicing)]
pub async fn init_postgres(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), PostgresInitError> {
    let db = PgPoolOptions::new()
        .max_connections(providers_json_local_data_hashmap.len() as u32)
        .connect_timeout(Duration::from_millis(10000)) //todo add timeout constant or env var
        .connect(&postgres_get_db_url())
        .await?;
    postgres_create_providers_tables_if_not_exists(&providers_json_local_data_hashmap, &db).await?;
    postgres_check_provider_links_tables_are_empty(&providers_json_local_data_hashmap, &db).await?;
    postgres_delete_all_from_providers_tables(&providers_json_local_data_hashmap, &db).await?;
    postgres_insert_link_parts_into_providers_tables(&providers_json_local_data_hashmap, &db)
        .await?;
    Ok(())
}
