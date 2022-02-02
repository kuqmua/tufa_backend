use std::collections::HashMap;

use sqlx::{Pool, Postgres};

use futures::future::join_all;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[derive(Debug)]
pub struct PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError {
    pub source:
        Box<PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum>,
            file: &'static str,
        line: u32,
        column: u32,
}

#[derive(Debug)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum {
    SelectCount(HashMap<ProviderKind, sqlx::Error>),
    ProviderLinksTablesRowsLengthNotEqual(
        HashMap<ProviderKind, ProviderLinksTablesLengthRowsNotEqualInitializationDataLength>,
    ),
}

#[derive(Debug)]
pub struct ProviderLinksTablesLengthRowsNotEqualInitializationDataLength {
    table_rows_length: i64,
    initialization_data_length: usize,
}

pub async fn postgres_check_providers_links_tables_length_rows_equal_initialization_data_length(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    db: &Pool<Postgres>,
) -> Result<(), PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError> {
    let count_provider_links_tables_tasks_vec =
        providers_json_local_data_hashmap
            .iter()
            .map(|(pk, string_vec)| async move {
                let query_string = format!(
                    "SELECT count(*) AS exact_count FROM {};",
                    pk.get_postgres_table_name()
                );
                (
                    pk,
                    string_vec,
                    sqlx::query_as(&query_string).fetch_one(db).await,
                )
            });
    let count_provider_links_tables_error_vec: Vec<(
        &ProviderKind,
        &Vec<String>,
        Result<(i64,), sqlx::Error>,
    )> = join_all(count_provider_links_tables_tasks_vec).await;
    let mut count_provider_links_tables_error_hashmap: HashMap<ProviderKind, sqlx::Error> =
        HashMap::new();
    let mut provider_links_tables_rows_length_not_equal_error_hashmap: HashMap<
        ProviderKind,
        ProviderLinksTablesLengthRowsNotEqualInitializationDataLength,
    > = HashMap::new();
    for (pk, string_vec, result) in count_provider_links_tables_error_vec {
        match result {
            Err(e) => {
                count_provider_links_tables_error_hashmap.insert(*pk, e);
            }
            Ok((count,)) => {
                if count != string_vec.len() as i64 {
                    provider_links_tables_rows_length_not_equal_error_hashmap.insert(
                        *pk,
                        ProviderLinksTablesLengthRowsNotEqualInitializationDataLength {
                            table_rows_length: count,
                            initialization_data_length: string_vec.len(),
                        },
                    );
                }
            }
        }
    }
    if !count_provider_links_tables_error_hashmap.is_empty() {
        return Err(PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError {
            source: Box::new(
                PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::SelectCount(
                    count_provider_links_tables_error_hashmap,
                ),
            ),
                        file: file!(),
            line: line!(),
            column: column!(),
        });
    }
    if !provider_links_tables_rows_length_not_equal_error_hashmap.is_empty() {
        return Err(PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError {
            source: Box::new(
                PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::ProviderLinksTablesRowsLengthNotEqual(
                    provider_links_tables_rows_length_not_equal_error_hashmap,
                ),
            ),
                        file: file!(),
            line: line!(),
            column: column!(),
        });
    }
    Ok(())
}
