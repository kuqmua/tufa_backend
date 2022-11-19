use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::mongo_client_options::MONGO_CLIENT_OPTIONS;
use crate::net_check::net_check_availability::net_check_availability;
use crate::net_check::net_check_availability::NetCheckAvailabilityWrapperError;
use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityOriginError;
use futures::join;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use std::ops::Deref;
use tufa_common::common::where_was::WhereWas;
use tufa_common::config_mods::traits::get_postgres_url::GetPostgresUrl;
use tufa_common::server::mongo::mongo_check_availability::mongo_check_availability;
use tufa_common::server::mongo::mongo_check_availability::MongoCheckAvailabilityWrapperError;
use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;

#[derive(
    Debug,
    ImplGetSourceFromTufaCommon,
    ImplDisplayForSimpleErrorEnum,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingFromTufaCommon,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
)]
pub struct CheckAvailabilityWrapperError {
    source: CheckAvailabilityWrapperErrorEnum,
    where_was: WhereWas,
}

#[allow(clippy::enum_variant_names)]
#[derive(
    Debug,
    ImplGetSourceFromTufaCommon,
    ImplDisplayForSimpleErrorEnum,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
)]
pub enum CheckAvailabilityWrapperErrorEnum {
    NetWrapper(Box<NetCheckAvailabilityWrapperError>),
    PostgresWrapper(Box<PostgresCheckAvailabilityOriginError>),
    MongoWrapper(Box<MongoCheckAvailabilityWrapperError>),
    NetAndMongoWrapper {
        net_source: Box<NetCheckAvailabilityWrapperError>,
        mongo_source: Box<MongoCheckAvailabilityWrapperError>,
    },
    NetAndPostgresWrapper {
        net_source: Box<NetCheckAvailabilityWrapperError>,
        postgres_source: Box<PostgresCheckAvailabilityOriginError>,
    },
    MongoAndPostgresWrapper {
        mongo_source: Box<MongoCheckAvailabilityWrapperError>,
        postgres_source: Box<PostgresCheckAvailabilityOriginError>,
    },
    NetAndMongoAndPostgresWrapper {
        net_source: Box<NetCheckAvailabilityWrapperError>,
        mongo_source: Box<MongoCheckAvailabilityWrapperError>,
        postgres_source: Box<PostgresCheckAvailabilityOriginError>,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn check_availability(
    should_trace: bool,
) -> Result<(), Box<CheckAvailabilityWrapperError>> {
    let net_url = &CONFIG.starting_check_link.clone();
    let postgres_url = &CONFIG.get_postgres_url();
    match join!(
        net_check_availability(net_url, false),
        postgres_check_availability(postgres_url, false),
        mongo_check_availability(
            MONGO_CLIENT_OPTIONS.deref().to_owned(), //std::time::Duration::from_millis(CONFIG.mongo_connection_timeout),
            &CONFIG.mongo_providers_logs_db_name,
            &CONFIG.source_place_type,
            false,
        ),
    ) {
        (Ok(_), Ok(_), Ok(_)) => Ok(()),
        (Ok(_), Ok(_), Err(m)) => Err(Box::new(
            CheckAvailabilityWrapperError::init_error_with_possible_trace(
                CheckAvailabilityWrapperErrorEnum::MongoWrapper(m),
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
        (Ok(_), Err(p), Ok(_)) => Err(Box::new(
            CheckAvailabilityWrapperError::init_error_with_possible_trace(
                CheckAvailabilityWrapperErrorEnum::PostgresWrapper(p),
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
        (Ok(_), Err(p), Err(m)) => Err(Box::new(
            CheckAvailabilityWrapperError::init_error_with_possible_trace(
                CheckAvailabilityWrapperErrorEnum::MongoAndPostgresWrapper {
                    mongo_source: m,
                    postgres_source: p,
                },
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
        (Err(n), Ok(_), Ok(_)) => Err(Box::new(
            CheckAvailabilityWrapperError::init_error_with_possible_trace(
                CheckAvailabilityWrapperErrorEnum::NetWrapper(n),
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
        (Err(n), Ok(_), Err(m)) => Err(Box::new(
            CheckAvailabilityWrapperError::init_error_with_possible_trace(
                CheckAvailabilityWrapperErrorEnum::NetAndMongoWrapper {
                    net_source: n,
                    mongo_source: m,
                },
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
        (Err(n), Err(p), Ok(_)) => Err(Box::new(
            CheckAvailabilityWrapperError::init_error_with_possible_trace(
                CheckAvailabilityWrapperErrorEnum::NetAndPostgresWrapper {
                    net_source: n,
                    postgres_source: p,
                },
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
        (Err(n), Err(p), Err(m)) => Err(Box::new(
            CheckAvailabilityWrapperError::init_error_with_possible_trace(
                CheckAvailabilityWrapperErrorEnum::NetAndMongoAndPostgresWrapper {
                    net_source: n,
                    postgres_source: p,
                    mongo_source: m,
                },
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
    }
}
