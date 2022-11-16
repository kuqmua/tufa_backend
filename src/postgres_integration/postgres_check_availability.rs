use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperWithMethodFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use std::time::Duration;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;

#[derive(
    Debug,
    ImplDisplayForErrorStruct,
    ImplGetSourceFromTufaCommon,
    ImplGetWhereWasOriginOrWrapperWithMethodFromTufaCommon,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon,
)]
pub struct PostgresCheckAvailabilityOriginError {
    source: Error,
    where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_check_availability(
    postgres_url: &str,
    should_trace: bool,
) -> Result<(), Box<PostgresCheckAvailabilityOriginError>> {
    if let Err(e) = PgPoolOptions::new()
        .max_connections(1)
        .connect_timeout(Duration::from_millis(CONFIG.postgres_connection_timeout))
        .connect(postgres_url)
        .await
    {
        return Err(Box::new(
            PostgresCheckAvailabilityOriginError::init_error_with_possible_trace(
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
        ));
    }
    Ok(())
}
