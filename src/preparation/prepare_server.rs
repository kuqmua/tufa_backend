pub async fn prepare_server<'a, SelfGeneric>(
    config: &'a (
        impl tufa_common::traits::config_fields::GetStartingCheckLink
        + tufa_common::traits::get_postgres_url::GetPostgresUrl<SelfGeneric>
        + tufa_common::traits::config_fields::GetPostgresConnectionTimeout
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
    //todo - check all postgres tables\requests to postgres. like if no some table - panic
    println!("service prepared!");
    Ok(())
}
