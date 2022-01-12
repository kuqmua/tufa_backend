use std::collections::HashMap;
use std::time::Duration;
use std::fmt;

use sqlx::postgres::PgPoolOptions;

use futures::future::join_all;

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
    InsertQueries(HashMap<ProviderKind, sqlx::Error>),
    EstablishConnection(sqlx::Error),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_postgres(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), PostgresInitError> {
    let db = PgPoolOptions::new()
    .max_connections(providers_json_local_data_hashmap.len() as u32)
    .connect_timeout(Duration::from_millis(1000))//todo add timeout constant or env var
    .connect(&postgres_get_db_url()).await?;
    // let mut tasks_vec = Vec::with_capacity(providers_json_local_data_hashmap.len());
    let tasks_vec = providers_json_local_data_hashmap.iter().map(|(pk, string_vec)|{
        async {
            let mut values_string = String::from("");
            for link_part in string_vec.clone() {
                values_string.push_str(&format!("('{}'),", link_part));
            }
            if !values_string.is_empty() {
                values_string.pop();
            }
            let query_string = format!("INSERT INTO providers_link_parts (link_part) VALUES {};", values_string);
            (*pk, sqlx::query(&query_string).execute(&db).await)
            }
    });
    let result_vec = join_all(tasks_vec).await; //todo: add state of success/unsuccess
    let mut error_hashmap = HashMap::new();
    for (pk, result) in result_vec {
        if let Err(e) = result {
            error_hashmap.insert(pk, e);
        }
    }
    if !error_hashmap.is_empty() {
        return Err(PostgresInitError { source: Box::new(PostgresInitErrorEnum::InsertQueries(error_hashmap))})
    }
    Ok(())
}   