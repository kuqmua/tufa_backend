pub async fn check_availability<'a>(
    should_trace: bool,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed<'a>>>{
    match futures::join!(
        tufa_common::server::net::net_check_availability::net_check_availability(&crate::global_variables::runtime::config::CONFIG.starting_check_link),
        tufa_common::server::postgres::postgres_check_availability::postgres_check_availability(
            {
                use tufa_common::traits::get_postgres_url::GetPostgresUrl;
                crate::global_variables::runtime::config::CONFIG.get_postgres_url()
            }, 
            crate::global_variables::runtime::config::CONFIG.postgres_connection_timeout
        ),
        tufa_common::server::mongo::mongo_check_availability::mongo_check_availability(
            {
                use std::ops::Deref;
                crate::global_variables::runtime::mongo_client_options::MONGO_CLIENT_OPTIONS.deref().to_owned() //todo timeout std::time::Duration::from_millis(CONFIG.mongo_connection_timeout),
            },
            &crate::global_variables::runtime::config::CONFIG.mongo_providers_logs_db_name,
        ),
    ) {
        (Ok(_), Ok(_), Ok(_)) => Ok(()),
        (Ok(_), Ok(_), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::Mongo {
            mongo: *m,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Ok(_), Err(p), Ok(_)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::Postgres {
            postgres: *p,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Ok(_), Err(p), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::PostgresAndMongo {
            postgres: *p,
            mongo: *m,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Err(n), Ok(_), Ok(_)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::Net {
            net: *n,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Err(n), Ok(_), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::NetAndMongo {
            net: *n,
            mongo: *m,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Err(n), Err(p), Ok(_)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::NetAndPostgres {
            net: *n,
            postgres: *p,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Err(n), Err(p), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityErrorNamed::NetAndPostgresAndMongo {
            net: *n,
            postgres: *p,
            mongo: *m,
            code_occurence: tufa_common::code_occurence!(),
        })),
    }
}