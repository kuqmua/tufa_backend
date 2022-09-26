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
pub struct PostgresCheckProvidersLinkPartsTablesEmptyError {
    source: PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceForEnumWithoutMethod)]
pub enum PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum {
    SelectCount(HashMap<ProviderKind, sqlx::Error>),
    NotEmpty(HashMap<ProviderKind, i64>),
}

impl std::fmt::Display for PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match crate::lazy_static::config::CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => {
                let mut formatted = match self {
                    PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum::SelectCount(hm) => hm
                        .iter()
                        .map(|(pk, error)| format!("{} {},", pk, error))
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        }),
                    PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum::NotEmpty(hm) => hm
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
                write!(f, "{}", formatted)
            }
        }
    }
}

impl tufa_common::traits::get_source::GetSource
    for PostgresCheckProvidersLinkPartsTablesEmptyError
{
    fn get_source(&self) -> String {
        let mut formatted = match &self.source {
            PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum::SelectCount(hm) => hm
                .iter()
                .map(|(pk, error)| format!("{} {},", pk, error))
                .fold(String::from(""), |mut acc, elem| {
                    acc.push_str(&elem);
                    acc
                }),
            PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum::NotEmpty(hm) => hm
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

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_check_providers_link_parts_tables_are_empty(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    db: &Pool<Postgres>,
    should_trace: bool,
) -> Result<(), Box<PostgresCheckProvidersLinkPartsTablesEmptyError>> {
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
        HashMap::with_capacity(count_provider_links_tables_error_vec.len());
    let mut provider_links_tables_not_empty_error_hashmap: HashMap<ProviderKind, i64> =
        HashMap::with_capacity(count_provider_links_tables_error_vec.len());
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
        return Err(Box::new(
            PostgresCheckProvidersLinkPartsTablesEmptyError::init_error_with_possible_trace(
                PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum::SelectCount(
                    count_provider_links_tables_error_hashmap,
                ),
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
    if !provider_links_tables_not_empty_error_hashmap.is_empty() {
        return Err(Box::new(
            PostgresCheckProvidersLinkPartsTablesEmptyError::init_error_with_possible_trace(
                PostgresCheckProvidersLinkPartsTablesEmptyErrorEnum::NotEmpty(
                    provider_links_tables_not_empty_error_hashmap,
                ),
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
