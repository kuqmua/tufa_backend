use std::collections::HashMap;
use std::time::Duration;
use std::fmt;

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use futures::future::join_all;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;
use crate::postgres_integration::postgres_create_providers_tables::PostgresCreateProvidersDbsError;
use crate::postgres_integration::postgres_create_providers_tables::postgres_create_providers_tables;

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct PostgresInitError {
    pub source: Box<PostgresInitErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum PostgresInitErrorEnum {
    // LoadingProvidersLinkParts(diesel::result::Error),
    // ProvidersLinkPartsIsNotEmpty(i64),
    // InsertPosts(diesel::result::Error),
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
    println!("providers_json_local_data_hashmap {:#?}", providers_json_local_data_hashmap);
    let db = PgPoolOptions::new()
    .max_connections(providers_json_local_data_hashmap.len() as u32)
    .connect_timeout(Duration::from_millis(10000))//todo add timeout constant or env var
    .connect(&postgres_get_db_url()).await?;
    postgres_create_providers_tables(&providers_json_local_data_hashmap, &db).await?;
    postgres_check_provider_links_tables_are_empty(&providers_json_local_data_hashmap, &db).await;
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

#[derive(Debug)]
pub struct PostgresCheckProvidersLinkPartsTablesEmptyError {
    pub source: Box<PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum>,
}

#[derive(Debug)]
pub enum PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum {
    SelectCount(HashMap<ProviderKind, sqlx::Error>),
    EstablishConnection(sqlx::Error),
}

pub async fn postgres_check_provider_links_tables_are_empty(providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>, db: &Pool<Postgres>) -> Result<(), PostgresCheckProvidersLinkPartsTablesEmptyError> {
    let count_provider_links_tables_tasks_vec = providers_json_local_data_hashmap.keys().map(|pk|{
        async {
            let query_string = format!("SELECT count(*) AS exact_count FROM {};", pk.get_postgres_table_name());
            let (count,): (i64,) = sqlx::query_as(&query_string).fetch_one(db).await.unwrap();
                (*pk, count)
            }
    });
    let count_provider_links_tables_error_hashmap = join_all(count_provider_links_tables_tasks_vec).await;
    // .into_iter().map(|(pk, result)| {
    //     if let Err(e) = result {
    //         return Some((pk, e));
    //     }
    //     None
    // }).collect::<HashMap<ProviderKind, sqlx::Error>>();
    println!("count_provider_links_tables_error_hashmap {:#?}", count_provider_links_tables_error_hashmap);
    // if !count_provider_links_tables_error_hashmap.is_empty() {
    //     return Err(PostgresCheckProvidersLinkPartsTablesEmptyError { source: Box::new(count_provider_links_tables_error_hashmap)})
    // }
    Ok(())
}