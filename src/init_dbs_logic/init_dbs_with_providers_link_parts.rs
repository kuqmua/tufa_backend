use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::init_dbs_logic::init_mongo::init_mongo;
use crate::init_dbs_logic::init_mongo::InitMongoWrapperError;
use crate::init_dbs_logic::init_postgres::init_postgres;
use crate::init_dbs_logic::init_postgres::PostgresInitWrapperError;
use crate::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use crate::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsWrapperError;
use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplGetSourceFromTufaCommon,
    ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
)]
pub struct InitDbsProvidersLinkPartsWrapperError {
    source: InitDbsProvidersLinkPartsWrapperErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetWhereWasOriginOrWrapperFromTufaCommon, ImplGetSourceFromTufaCommon)]
pub enum InitDbsProvidersLinkPartsWrapperErrorEnum {
    GetLocalProvidersLinkPartsWrapper(GetLocalProvidersLinkPartsWrapperError),
    PostgresInitWrapper(PostgresInitWrapperError),
    MongoInitWrapper(InitMongoWrapperError),
    //todo - do something with that - add support for fields
    // MongoAndPostgresInitOrigin {
    //     mongo: InitMongoError,
    //     postgres: PostgresInitError,
    // },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn init_dbs_with_providers_link_parts(
    should_trace: bool,
) -> Result<(), Box<InitDbsProvidersLinkPartsWrapperError>> {
    match get_local_providers_link_parts(false).await {
        Err(e) => Err(Box::new(
            InitDbsProvidersLinkPartsWrapperError::init_error_with_possible_trace(
                InitDbsProvidersLinkPartsWrapperErrorEnum::GetLocalProvidersLinkPartsWrapper(*e),
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
        Ok(providers_json_local_data_hashmap) => {
            let providers_json_local_data_hashmap_clone = providers_json_local_data_hashmap.clone();
            let (mongo_insert_data_option_result, postgres_insert_data_option_result) = tokio::join!(
                async {
                    match CONFIG.is_mongo_initialization_enabled {
                        false => None,
                        true => Some(init_mongo(providers_json_local_data_hashmap, false).await),
                    }
                },
                async {
                    match CONFIG.is_postgres_initialization_enabled {
                        false => None,
                        true => Some(
                            init_postgres(providers_json_local_data_hashmap_clone, false).await,
                        ),
                    }
                }
            );
            match (
                mongo_insert_data_option_result,
                postgres_insert_data_option_result,
            ) {
                (None, None) => (),
                (None, Some(pg_result)) => {
                    if let Err(e) = pg_result {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsWrapperError::init_error_with_possible_trace(
                                InitDbsProvidersLinkPartsWrapperErrorEnum::PostgresInitWrapper(*e),
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
                }
                (Some(mongo_result), None) => {
                    if let Err(e) = mongo_result {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsWrapperError::init_error_with_possible_trace(
                                InitDbsProvidersLinkPartsWrapperErrorEnum::MongoInitWrapper(*e),
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
                }
                (Some(mongo_result), Some(pg_result)) => match (mongo_result, pg_result) {
                    (Ok(_), Ok(_)) => (),
                    (Ok(_), Err(e)) => {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsWrapperError::init_error_with_possible_trace(
                                InitDbsProvidersLinkPartsWrapperErrorEnum::PostgresInitWrapper(*e),
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
                    (Err(e), Ok(_)) => {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsWrapperError::init_error_with_possible_trace(
                                InitDbsProvidersLinkPartsWrapperErrorEnum::MongoInitWrapper(*e),
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
                    (Err(mongo_error), Err(postgres_error)) => {
                        todo!();
                        // return Err(Box::new(
                        //     InitDbsProvidersLinkPartsError::init_error_with_possible_trace(
                        //         InitDbsProvidersLinkPartsWrapperErrorEnum::MongoAndPostgresInitOrigin {
                        //             mongo: *mongo_error,
                        //             postgres: *postgres_error,
                        //         },
                        //         WhereWas {
                        //             time: std::time::SystemTime::now()
                        //                 .duration_since(std::time::UNIX_EPOCH)
                        //                 .expect("cannot convert time to unix_epoch"),
                        //             location: *core::panic::Location::caller(),
                        //         },
                        //         &CONFIG.source_place_type,
                        //         &GIT_INFO,
                        //         should_trace,
                        //     ),
                        // ));
                    }
                },
            }
            Ok(())
        }
    }
}
