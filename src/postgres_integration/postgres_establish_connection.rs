use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use sqlx::Postgres;
use std::collections::HashMap;
use std::time::Duration;
use tufa_common::common::where_was::WhereWas;
use tufa_common::common::where_was::WhereWasOneOrMany;
use tufa_common::common::where_was::WhereWasWithAddition;
use tufa_common::config_mods::traits::get_postgres_url::GetPostgresUrl;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplGetSourceWithoutMethodFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
    ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon,
)]
pub struct PostgresEstablishConnectionError {
    pub source: Error,
    pub where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_establish_connection(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    should_trace: bool,
) -> Result<sqlx::Pool<Postgres>, Box<PostgresEstablishConnectionError>> {
    match PgPoolOptions::new()
        .max_connections(providers_json_local_data_hashmap.len() as u32)
        .connect_timeout(Duration::from_millis(CONFIG.postgres_connection_timeout)) //todo add timeout constant or env var
        .connect(&CONFIG.get_postgres_url())
        .await
    {
        Err(e) => Err(Box::new(
            PostgresEstablishConnectionError::init_error_with_possible_trace(
                e,
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
        )),
        Ok(pool) => Ok(pool),
    }
}
