use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
// use crate::init_dbs_logic::init_dbs::init_dbs;
// use crate::init_dbs_logic::init_dbs::InitDbsWrapperError;
use crate::preparation::check_availability::check_availability;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_git_info::ImplGetGitInfoFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;

pub async fn prepare_server<'a>(should_trace: bool) -> Result<(), Box<tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerError<'a>>> {
    if let Err(e) = check_availability(false).await {
        return Err(Box::new(
            tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerError::CheckAvailability { 
                inner_error: *e, 
                code_occurence: tufa_common::code_occurence!(),
            }
        ));
    }
    //todo: add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
    if !CONFIG.is_dbs_initialization_enabled
        || (!CONFIG.is_mongo_initialization_enabled && !CONFIG.is_postgres_initialization_enabled)
    {
        return Ok(());
    }
    // if let Err(e) = init_dbs(false).await {
    //     return Err(Box::new(
    //         PreparationWrapperError::init_error_with_possible_trace(
    //             InitDbsProvidersLinkPartsWrapperErrorEnum::InitDbsWrapper(*e),
    //             WhereWas {
    //                 time: std::time::SystemTime::now()
    //                     .duration_since(std::time::UNIX_EPOCH)
    //                     .expect("cannot convert time to unix_epoch"),
    //                 file: String::from(file!()),
    //                 line: line!(),
    //                 column: column!(),
    //                 git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
    //             },
    //             &CONFIG.source_place_type,
    //             should_trace,
    //         ),
    //     ));
    // }
    Ok(())
}
