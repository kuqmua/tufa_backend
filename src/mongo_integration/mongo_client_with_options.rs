use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplGetSourceFromTufaCommon,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
    ImplErrorWithTracingFromTufaCommon,
)]
pub struct MongoClientWithOptionsOriginError {
    source: Error,
    where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn mongo_client_with_options(
    client_options: ClientOptions,
    should_trace: bool,
) -> Result<Client, Box<MongoClientWithOptionsOriginError>> {
    match Client::with_options(client_options) {
        Err(e) => Err(Box::new(
            MongoClientWithOptionsOriginError::init_error_with_possible_trace(
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
        Ok(client) => Ok(client),
    }
}
