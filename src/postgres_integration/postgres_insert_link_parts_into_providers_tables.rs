use chrono::{DateTime, FixedOffset, Local, Utc};

use std::collections::HashMap;

use sqlx::{Pool, Postgres};

use futures::future::join_all;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::helpers::where_was::WhereWas;

#[derive(Debug)]
pub struct PostgresInsertLinkPartsIntoProvidersTablesError {
    pub source: Box<HashMap<ProviderKind, sqlx::Error>>,
    where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_insert_link_parts_into_providers_tables(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    pool: &Pool<Postgres>,
) -> Result<(), PostgresInsertLinkPartsIntoProvidersTablesError> {
    let insertion_error_hashmap = join_all(providers_json_local_data_hashmap.iter().map(
        |(pk, string_vec)| async {
            let mut values_string = String::from("");
            for link_part in string_vec.clone() {
                values_string.push_str(&format!("('{link_part}'),"));
            }
            if !values_string.is_empty() {
                values_string.pop();
            }
            let query_string = format!(
                "INSERT INTO {} (link_part) VALUES {values_string};",
                pk.get_postgres_table_name()
            );
            (*pk, sqlx::query(&query_string).execute(pool).await)
        },
    ))
    .await
    .into_iter()
    .filter_map(|(pk, result)| {
        if let Err(e) = result {
            return Some((pk, e));
        }
        None
    })
    .collect::<HashMap<ProviderKind, sqlx::Error>>();
    if !insertion_error_hashmap.is_empty() {
        return Err(PostgresInsertLinkPartsIntoProvidersTablesError {
            source: Box::new(insertion_error_hashmap),
            where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(3 * 3600)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        });
    }
    Ok(())
}
