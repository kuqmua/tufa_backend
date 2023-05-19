pub async fn prepare_server<'a, SelfGeneric>(
    config: &'static (
        impl tufa_common::traits::config_fields::GetStartingCheckLink
        + tufa_common::traits::get_postgres_url::GetPostgresUrl<SelfGeneric>
        + tufa_common::traits::config_fields::GetPostgresConnectionTimeout
        + std::marker::Send 
        + std::marker::Sync
    )
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerErrorNamed<'a>>> {
    if let Err(e) = tufa_common::repositories_types::tufa_server::preparation::check_availability::check_availability(
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
