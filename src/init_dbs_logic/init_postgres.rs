use std::collections::HashMap;
use std::time::Duration;
use std::fmt;

use sqlx::postgres::PgPoolOptions;


use crate::providers::provider_kind_enum::ProviderKind;

use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsError;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::postgres_create_providers_tables_if_not_exists;
use crate::postgres_integration::postgres_check_provider_links_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyError;
use crate::postgres_integration::postgres_check_provider_links_tables_are_empty::postgres_check_provider_links_tables_are_empty;

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct PostgresInitError {
    pub source: Box<PostgresInitErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum PostgresInitErrorEnum {
    // LoadingProvidersLinkParts(diesel::result::Error),
    // ProvidersLinkPartsIsNotEmpty(i64),
    // InsertPosts(diesel::result::Error),
    CheckProviderLinksTablesAreEmpty(PostgresCheckProvidersLinkPartsTablesEmptyError),
    CreateTableQueries(PostgresCreateProvidersDbsError),
    InsertQueries(InsertQueriesHashmap),
    EstablishConnection(sqlx::Error),
}

pub type CreateTableQueriesHashmap = HashMap<ProviderKind, sqlx::Error>;//for wroking logic for now. todo: move into different function
pub type InsertQueriesHashmap = HashMap<ProviderKind, sqlx::Error>;//for wroking logic for now. todo: move into different function
#[deny(clippy::indexing_slicing)]
pub async fn init_postgres(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), PostgresInitError> {
    let db = PgPoolOptions::new()
    .max_connections(providers_json_local_data_hashmap.len() as u32)
    .connect_timeout(Duration::from_millis(10000))//todo add timeout constant or env var
    .connect(&postgres_get_db_url()).await?;
    postgres_create_providers_tables_if_not_exists(&providers_json_local_data_hashmap, &db).await?;
    println!("providers_json_local_data_hashmap {:#?}", providers_json_local_data_hashmap);
    postgres_check_provider_links_tables_are_empty(&providers_json_local_data_hashmap, &db).await?;
    // let insertion_tasks_vec = providers_json_local_data_hashmap.iter().map(|(pk, string_vec)|{
    //     async {
    //         let mut values_string = String::from("");
    //         for link_part in string_vec.clone() {
    //             values_string.push_str(&format!("('{}'),", link_part));
    //         }
    //         if !values_string.is_empty() {
    //             values_string.pop();
    //         }
    //         let query_string = format!("INSERT INTO {} (link_part) VALUES {};", pk.get_postgres_table_name(), values_string);
    //         (*pk, sqlx::query(&query_string).execute(&db).await)
    //         }
    // });
    // let insertion_error_hashmap = join_all(insertion_tasks_vec).await.into_iter().filter_map(|(pk, result)| {
    //     if let Err(e) = result {
    //         return Some((pk, e));
    //     }
    //     None
    // }).collect::<HashMap<ProviderKind, sqlx::Error>>();
    // if !insertion_error_hashmap.is_empty() {
    //     return Err(PostgresInitError { source: Box::new(PostgresInitErrorEnum::InsertQueries(insertion_error_hashmap))})
    // }
    Ok(())
}   