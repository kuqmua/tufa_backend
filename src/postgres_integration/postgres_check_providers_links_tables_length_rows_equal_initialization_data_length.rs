use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use futures::future::join_all;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;

#[derive(
    Debug,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
    ImplErrorWithTracingFromTufaCommon,
    InitErrorFromTufaCommon,
    ImplGetSourceFromTufaCommon,
)]
pub struct PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperError {
    source: PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthOriginErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceFromTufaCommon, ImplGetWhereWasOriginOrWrapperFromTufaCommon)]
pub enum PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthOriginErrorEnum {
    SelectCountOrigin(HashMap<ProviderKind, sqlx::Error>),
    ProviderLinksTablesRowsLengthNotEqualOrigin(
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

pub async fn postgres_check_providers_links_tables_length_rows_equal_initialization_data_length(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    db: &Pool<Postgres>,
    should_trace: bool,
) -> Result<
    (),
    Box<PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperError>,
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
            PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperError::init_error_with_possible_trace(
                PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthOriginErrorEnum::SelectCountOrigin(count_provider_links_tables_error_hashmap),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                },
                &CONFIG.source_place_type,
                should_trace,
            ),
        ));
    }
    if !provider_links_tables_rows_length_not_equal_error_hashmap.is_empty() {
        return Err(Box::new(
            PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthWrapperError::init_error_with_possible_trace(
                PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthOriginErrorEnum::ProviderLinksTablesRowsLengthNotEqualOrigin(provider_links_tables_rows_length_not_equal_error_hashmap),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                },
                &CONFIG.source_place_type,
                should_trace,
            ),
        ));
    }
    Ok(())
}
