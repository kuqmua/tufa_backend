use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use crate::global_variables::runtime::mongo_client_options::MONGO_CLIENT_OPTIONS;
use futures::join;
use impl_display_for_error::ImplDisplayForError;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_git_info::ImplGetGitInfoFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use std::ops::Deref;
use tufa_common::server::mongo::mongo_check_availability::mongo_check_availability;
use tufa_common::server::net::net_check_availability::net_check_availability;
use tufa_common::server::postgres::postgres_check_availability::postgres_check_availability;
use tufa_common::traits::get_postgres_url::GetPostgresUrl;

pub async fn check_availability<'a>(
    should_trace: bool,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError<'a>>>{
    let net_url = CONFIG.starting_check_link.clone();
    let postgres_url = CONFIG.get_postgres_url();
    match join!(
        net_check_availability(net_url),
        postgres_check_availability(postgres_url, false),
        mongo_check_availability(
            MONGO_CLIENT_OPTIONS.deref().to_owned(), //std::time::Duration::from_millis(CONFIG.mongo_connection_timeout),
            &CONFIG.mongo_providers_logs_db_name,
            &CONFIG.source_place_type,
            false,
        ),
    ) {
        (Ok(_), Ok(_), Ok(_)) => Ok(()),
        (Ok(_), Ok(_), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::Mongo {
            mongo: *m,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Ok(_), Err(p), Ok(_)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::Postgres {
            postgres: *p,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Ok(_), Err(p), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::PostgresAndMongo {
            postgres: *p,
            mongo: *m,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Err(n), Ok(_), Ok(_)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::Net {
            net: *n,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Err(n), Ok(_), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::NetAndMongo {
            net: *n,
            mongo: *m,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Err(n), Err(p), Ok(_)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::NetAndPostgres {
            net: *n,
            postgres: *p,
            code_occurence: tufa_common::code_occurence!(),
        })),
        (Err(n), Err(p), Err(m)) => Err(Box::new(tufa_common::repositories_types::tufa_server::preparation::check_availability::CheckAvailabilityError::NetAndPostgresAndMongo {
            net: *n,
            postgres: *p,
            mongo: *m,
            code_occurence: tufa_common::code_occurence!(),
        })),
    }
}