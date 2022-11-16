use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::init_dbs_logic::init_dbs::init_dbs;
use crate::init_dbs_logic::init_dbs::InitDbsWrapperError;
use crate::preparation::check_availability::check_availability;
use crate::preparation::check_availability::CheckAvailabilityWrapperError;
use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethodFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon,
    ImplGetWhereWasOneOrManyWithMethodFromTufaCommon,
)]
pub struct PreparationWrapperError {
    source: PreparationErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetWhereWasOneOrManyWithMethodFromTufaCommon, ImplGetSourceFromTufaCommon)]
pub enum PreparationErrorEnum {
    CheckAvailabilityWrapper(CheckAvailabilityWrapperError),
    InitDbsWrapper(InitDbsWrapperError),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn prepare_server(should_trace: bool) -> Result<(), Box<PreparationWrapperError>> {
    if let Err(e) = check_availability(false).await {
        return Err(Box::new(
            PreparationWrapperError::init_error_with_possible_trace(
                PreparationErrorEnum::CheckAvailabilityWrapper(*e),
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
    //todo: add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
    if !CONFIG.is_dbs_initialization_enabled
        || (!CONFIG.is_mongo_initialization_enabled && !CONFIG.is_postgres_initialization_enabled)
    {
        return Ok(());
    }
    if let Err(e) = init_dbs(false).await {
        return Err(Box::new(
            PreparationWrapperError::init_error_with_possible_trace(
                PreparationErrorEnum::InitDbsWrapper(*e),
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
