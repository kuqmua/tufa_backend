pub async fn prepare_server<'a>() -> Result<(), Box<tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerErrorNamed<'a>>> {
    if let Err(e) = tufa_common::repositories_types::tufa_server::preparation::check_availability::check_availability().await {
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
    Ok(())
}
