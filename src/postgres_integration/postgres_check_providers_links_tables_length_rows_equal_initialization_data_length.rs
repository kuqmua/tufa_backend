use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use impl_get_source_for_enum_without_method::ImplGetSourceForEnumWithoutMethod;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use init_error_with_tracing_for_original_error_struct::InitErrorWithTracingForOriginalErrorStruct;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
    InitErrorWithTracingForOriginalErrorStruct,
    InitError,
)]
pub struct PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError {
    source: PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceForEnumWithoutMethod)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum {
    SelectCount(HashMap<ProviderKind, sqlx::Error>),
    ProviderLinksTablesRowsLengthNotEqual(
        HashMap<ProviderKind, ProviderLinksTablesLengthRowsNotEqualInitializationDataLength>,
    ),
}

impl std::fmt::Display
    for PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut formatted = match self {
            PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::SelectCount(hm) => {
                hm
                    .iter()
                    .map(|(pk, error)| format!("{} {},", pk, error))
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                })
            },
            PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::ProviderLinksTablesRowsLengthNotEqual(hm) => {
                hm
                    .iter()
                    .map(|(pk, error)| format!("{} {},", pk, error))
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                })
            },
        };
        if !formatted.is_empty() {
            formatted.pop();
        }
        write!(f, "{}", formatted)
    }
}

#[derive(Debug)]
pub struct ProviderLinksTablesLengthRowsNotEqualInitializationDataLength {
    pub table_rows_length: i64,
    pub initialization_data_length: usize,
}

impl std::fmt::Display for ProviderLinksTablesLengthRowsNotEqualInitializationDataLength {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "table_rows_length: {}, initialization_data_length: {}",
            self.table_rows_length, self.initialization_data_length
        )
    }
}

impl tufa_common::traits::get_source::GetSource
    for PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError
{
    fn get_source(&self) -> String {
        self.source.get_source()
    }
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
    should_trace: bool,
) -> Result<(), Box<PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError>>
{
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
            PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError::init_error_with_possible_trace(
                PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::SelectCount(count_provider_links_tables_error_hashmap),
                WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        ));
    }
    if !provider_links_tables_rows_length_not_equal_error_hashmap.is_empty() {
        return Err(Box::new(
            PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError::init_error_with_possible_trace(
                PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::ProviderLinksTablesRowsLengthNotEqual(provider_links_tables_rows_length_not_equal_error_hashmap),
                WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        ));
    }
    Ok(())
}
