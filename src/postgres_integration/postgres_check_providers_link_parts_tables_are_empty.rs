use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use futures::future::join_all;
use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon;
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
    ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon,
    InitErrorFromTufaCommon,
    ImplGetSourceFromTufaCommon,
)]
pub struct PostgresCheckProvidersLinkPartsTablesEmptyWrapperError {
    source: PostgresCheckProvidersLinkPartsTablesEmptyOriginErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceFromTufaCommon)]
pub enum PostgresCheckProvidersLinkPartsTablesEmptyOriginErrorEnum {
    SelectCountOrigin(HashMap<ProviderKind, sqlx::Error>),
    NotEmptyOrigin(HashMap<ProviderKind, i64>),
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
) -> Result<(), Box<PostgresCheckProvidersLinkPartsTablesEmptyWrapperError>> {
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
            PostgresCheckProvidersLinkPartsTablesEmptyWrapperError::init_error_with_possible_trace(
                PostgresCheckProvidersLinkPartsTablesEmptyOriginErrorEnum::SelectCountOrigin(
                    count_provider_links_tables_error_hashmap,
                ),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO,
                should_trace,
            ),
        ));
    }
    if !provider_links_tables_not_empty_error_hashmap.is_empty() {
        return Err(Box::new(
            PostgresCheckProvidersLinkPartsTablesEmptyWrapperError::init_error_with_possible_trace(
                PostgresCheckProvidersLinkPartsTablesEmptyOriginErrorEnum::NotEmptyOrigin(
                    provider_links_tables_not_empty_error_hashmap,
                ),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO,
                should_trace,
            ),
        ));
    }
    Ok(())
}
