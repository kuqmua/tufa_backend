use std::collections::HashMap;

use sqlx::{Pool, Postgres};

use futures::future::join_all;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[derive(Debug)]
pub struct PostgresCheckProvidersLinkPartsTablesEmptyError {
    pub source: Box<PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum>,
}

#[derive(Debug)]
pub enum PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum {
    SelectCount(HashMap<ProviderKind, sqlx::Error>),
    NotEmpty(HashMap<ProviderKind, i64>),
}

pub async fn postgres_check_provider_links_tables_are_empty(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    db: &Pool<Postgres>,
) -> Result<(), PostgresCheckProvidersLinkPartsTablesEmptyError> {
    let count_provider_links_tables_tasks_vec =
        providers_json_local_data_hashmap.keys().map(|pk| async {
            let query_string = format!(
                "SELECT count(*) AS exact_count FROM {};",
                pk.get_postgres_table_name()
            );
            (*pk, sqlx::query_as(&query_string).fetch_one(db).await)
        });
    let count_provider_links_tables_error_vec: Vec<(ProviderKind, Result<(i64,), sqlx::Error>)> =
        join_all(count_provider_links_tables_tasks_vec).await;
    let mut count_provider_links_tables_error_hashmap: HashMap<ProviderKind, sqlx::Error> =
        HashMap::new();
    let mut provider_links_tables_not_empty_error_hashmap: HashMap<ProviderKind, i64> =
        HashMap::new();
    for (pk, result) in count_provider_links_tables_error_vec {
        match result {
            Err(e) => {
                count_provider_links_tables_error_hashmap.insert(pk, e);
            }
            Ok((count,)) => {
                if count > 0 {
                    provider_links_tables_not_empty_error_hashmap.insert(pk, count);
                }
            }
        }
    }
    if !count_provider_links_tables_error_hashmap.is_empty() {
        return Err(PostgresCheckProvidersLinkPartsTablesEmptyError {
            source: Box::new(
                PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum::SelectCount(
                    count_provider_links_tables_error_hashmap,
                ),
            ),
        });
    }
    if !provider_links_tables_not_empty_error_hashmap.is_empty() {
        return Err(PostgresCheckProvidersLinkPartsTablesEmptyError {
            source: Box::new(
                PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum::NotEmpty(
                    provider_links_tables_not_empty_error_hashmap,
                ),
            ),
        });
    }
    Ok(())
}
