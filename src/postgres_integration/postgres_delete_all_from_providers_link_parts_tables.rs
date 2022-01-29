use std::collections::HashMap;

use sqlx::{Pool, Postgres};

use futures::future::join_all;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[derive(Debug)]
pub struct PostgresDeleteAllFromProvidersTablesError {
    pub source: Box<HashMap<ProviderKind, sqlx::Error>>,
    line: String,
}

pub async fn postgres_delete_all_from_providers_link_parts_tables(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    pool: &Pool<Postgres>,
) -> Result<(), PostgresDeleteAllFromProvidersTablesError> {
    let delete_from_tables_tasks_vec = providers_json_local_data_hashmap.keys().map(|pk| async {
        let query_string = format!("DELETE FROM {} ;", pk.get_postgres_table_name());
        (*pk, sqlx::query(&query_string).execute(pool).await)
    });
    let delete_from_tables_error_hashmap = join_all(delete_from_tables_tasks_vec)
        .await
        .into_iter()
        .filter_map(|(pk, result)| {
            if let Err(e) = result {
                return Some((pk, e));
            }
            None
        })
        .collect::<HashMap<ProviderKind, sqlx::Error>>();
    if !delete_from_tables_error_hashmap.is_empty() {
        return Err(PostgresDeleteAllFromProvidersTablesError {
            source: Box::new(delete_from_tables_error_hashmap),
            line: format!("{}:{}:{}", line!(), file!(), column!()),
        });
    }
    Ok(())
}
