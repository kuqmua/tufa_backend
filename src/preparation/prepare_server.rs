// use crate::init_dbs_logic::init_dbs::init_dbs;
// use crate::init_dbs_logic::init_dbs::InitDbsWrapperError;
pub async fn prepare_server<'a>(should_trace: bool) -> Result<(), Box<tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerErrorNamed<'a>>> {
    if let Err(e) = crate::preparation::check_availability::check_availability(false).await {
        return Err(Box::new(
            tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerErrorNamed::CheckAvailability { 
                check_availability: *e, 
                code_occurence: tufa_common::code_occurence!(),
            }
        ));
    }
    //todo: add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
    if !crate::global_variables::runtime::config::CONFIG.is_dbs_initialization_enabled
        || (!crate::global_variables::runtime::config::CONFIG.is_mongo_initialization_enabled && !crate::global_variables::runtime::config::CONFIG.is_postgres_initialization_enabled)
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
    //             &crate::global_variables::runtime::config::CONFIG.source_place_type,
    //             should_trace,
    //         ),
    //     ));
    // }
    Ok(())
}
