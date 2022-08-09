use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;

#[derive(Debug)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum {
    SelectCount {
        source: HashMap<ProviderKind, sqlx::Error>,
        where_was: WhereWas,
    },
    ProviderLinksTablesRowsLengthNotEqual {
        source:
            HashMap<ProviderKind, ProviderLinksTablesLengthRowsNotEqualInitializationDataLength>,
        where_was: WhereWas,
    },
}

#[derive(Debug)]
pub struct ProviderLinksTablesLengthRowsNotEqualInitializationDataLength {
    pub table_rows_length: i64,
    pub initialization_data_length: usize,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_check_providers_links_tables_length_rows_equal_initialization_data_length(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    db: &Pool<Postgres>,
) -> Result<
    (),
    Box<PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum>,
> {
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
        return Err(Box::new(
            PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::SelectCount {
                source: count_provider_links_tables_error_hashmap,
                where_was: WhereWas::new(
                DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file!(),
                line!(),
                column!(),
                None
            ),
            },
        ));
    }
    if !provider_links_tables_rows_length_not_equal_error_hashmap.is_empty() {
        return Err(Box::new(
            PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::ProviderLinksTablesRowsLengthNotEqual {
                source: provider_links_tables_rows_length_not_equal_error_hashmap,
                where_was: WhereWas::new(
                DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file!(),
                line!(),
                column!(),
                None
            ),
            },
        ));
    }
    Ok(())
}
