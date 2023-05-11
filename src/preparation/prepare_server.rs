pub async fn prepare_server<'a, SelfGeneric>(
    config: &'a (
        impl tufa_common::traits::config_fields::GetStartingCheckLink
        + tufa_common::traits::get_postgres_url::GetPostgresUrl<SelfGeneric>
        + tufa_common::traits::config_fields::GetPostgresConnectionTimeout
        + tufa_common::traits::config_fields::GetMongoProvidersLogsDbName

        + tufa_common::traits::config_fields::GetIsDbsInitializationEnabled
        + tufa_common::traits::config_fields::GetIsMongoInitializationEnabled
        + tufa_common::traits::config_fields::GetIsPostgresInitializationEnabled
    )
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerErrorNamed<'a>>> {
    if let Err(e) = tufa_common::repositories_types::tufa_server::preparation::check_availability::check_availability(
        {
            use std::ops::Deref;
            crate::global_variables::runtime::mongo_client_options::MONGO_CLIENT_OPTIONS.deref().to_owned()
        },
        config,
    ).await {
        return Err(Box::new(
            tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerErrorNamed::CheckAvailability { 
                check_availability: *e, 
                code_occurence: tufa_common::code_occurence!(),
            }
        ));
    }
    //todo: add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
    if 
    !config.get_is_dbs_initialization_enabled()
    || 
    (
        !config.get_is_mongo_initialization_enabled()
        && 
        !config.get_is_postgres_initialization_enabled()
    )
    {
        return Ok(());
    }
    Ok(())
}
