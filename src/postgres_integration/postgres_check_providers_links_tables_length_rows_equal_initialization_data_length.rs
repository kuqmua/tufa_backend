use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::with_tracing::WithTracing;
use tufa_common::where_was::WhereWas;

#[derive(Debug)] //, ImplGetWhereWasForErrorStruct
pub struct PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError {
    source: PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum,
    where_was: WhereWas,
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
    for PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError
{
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        tufa_common::where_was::WhereWasOneOrMany::One(
            tufa_common::where_was::WhereWasWithAddition {
                additional_info: None,
                where_was: self.where_was.clone(),
            },
        )
    }
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
    for PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum
{
    fn get_source(&self) -> String {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self),
            false => {
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
                formatted
            }
        }
    }
}

impl
    tufa_common::traits::with_tracing::WithTracing<
        PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum,
    > for PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError
{
    fn with_tracing(
        source: PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum,
        where_was: WhereWas,
    ) -> Self {
        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = source.get_source(),
                    source_place = where_was.file_line_column(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = source.get_source(),
                    github_source_place = where_was
                        .github_file_line_column(&crate::lazy_static::git_info::GIT_INFO.data),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = source.get_source());
            }
        }
        Self { source, where_was }
    }
}
//todo implement better type support for derive(InitError)
impl PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError {
    pub fn new(
        source: PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum,
        where_was: WhereWas,
    ) -> Self {
        Self { source, where_was }
    }
}

impl tufa_common::traits::get_source::GetSource
    for PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError
{
    fn get_source(&self) -> String {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self.source),
            false => {
                let mut formatted = match &self.source {
                    PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::SelectCount(hm) => hm
                        .iter()
                        .map(|(pk, error)| format!("{} {},", pk, error))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        }),
                    PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::ProviderLinksTablesRowsLengthNotEqual(hm) => hm
                        .iter()
                        .map(|(pk, error)| format!("{} {},", pk, error))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        }),
                };
                if !formatted.is_empty() {
                    formatted.pop();
                }
                formatted
            }
        }
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
        let where_was = WhereWas {
            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file: file!(),
            line: line!(),
            column: column!(),
        };
        match should_trace {
            true => {
                return Err(Box::new(
                    PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError::with_tracing(
                        PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::SelectCount(count_provider_links_tables_error_hashmap),
                        where_was
                )));
            }
            false => {
                return Err(Box::new(
                    PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError::new(
                        PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::SelectCount(count_provider_links_tables_error_hashmap),
                        where_was
                )));
            }
        }
    }
    if !provider_links_tables_rows_length_not_equal_error_hashmap.is_empty() {
        let where_was = WhereWas {
            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file: file!(),
            line: line!(),
            column: column!(),
        };
        match should_trace {
            true => {
                return Err(Box::new(
                    PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError::with_tracing(
                        PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::ProviderLinksTablesRowsLengthNotEqual(provider_links_tables_rows_length_not_equal_error_hashmap),
                        where_was
                )));
            }
            false => {
                return Err(Box::new(
                    PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError::new(
                        PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthErrorEnum::ProviderLinksTablesRowsLengthNotEqual(provider_links_tables_rows_length_not_equal_error_hashmap),
                        where_was
                )));
            }
        }
    }
    Ok(())
}
