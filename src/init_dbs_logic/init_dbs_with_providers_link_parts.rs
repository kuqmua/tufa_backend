use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::init_dbs_logic::init_mongo::init_mongo;
use crate::init_dbs_logic::init_mongo::InitMongoError;
use crate::init_dbs_logic::init_postgres::init_postgres;
use crate::init_dbs_logic::init_postgres::PostgresInitError;
use crate::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use crate::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsError;
use crate::traits::get_source::GetSource;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;

#[derive(Debug, ImplGetWhereWasForErrorStruct, InitError)]
pub struct InitDbsProvidersLinkPartsError {
    source: InitDbsProvidersLinkPartsErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug)]
pub enum InitDbsProvidersLinkPartsErrorEnum {
    GetLocalProvidersLinkParts(GetLocalProvidersLinkPartsError),
    PostgresInit(PostgresInitError),
    MongoInit(InitMongoError),
    MongoAndPostgresInit {
        mongo: InitMongoError,
        postgres: PostgresInitError,
    },
}

impl crate::traits::get_source::GetSource for InitDbsProvidersLinkPartsErrorEnum {
    fn get_source(&self) -> String {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self),
            false => {
                let mut formatted = match self {
                    InitDbsProvidersLinkPartsErrorEnum::GetLocalProvidersLinkParts(e) => {
                        e.get_source()
                    }
                    InitDbsProvidersLinkPartsErrorEnum::PostgresInit(e) => e.get_source(),
                    InitDbsProvidersLinkPartsErrorEnum::MongoInit(e) => e.get_source(),
                    InitDbsProvidersLinkPartsErrorEnum::MongoAndPostgresInit {
                        mongo: mongo_error,
                        postgres: postgres_error,
                    } => format!(
                        "{} {}",
                        mongo_error.get_source(),
                        postgres_error.get_source()
                    ),
                };
                if !formatted.is_empty() {
                    formatted.pop();
                }
                formatted
            }
        }
    }
}

impl InitDbsProvidersLinkPartsError {
    pub fn with_tracing(source: InitDbsProvidersLinkPartsErrorEnum, where_was: WhereWas) -> Self {
        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = source.get_source(),
                    source_place = where_was.source_place(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = source.get_source(),
                    github_source_place = where_was.github_source_place(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = source.get_source());
            }
        }
        Self { source, where_was }
    }
}

impl crate::traits::get_source::GetSource for InitDbsProvidersLinkPartsError {
    fn get_source(&self) -> String {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self.source),
            false => self.source.get_source(),
        }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn init_dbs_with_providers_link_parts(
    should_trace: bool,
) -> Result<(), Box<InitDbsProvidersLinkPartsError>> {
    match get_local_providers_link_parts(false).await {
        Err(e) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(InitDbsProvidersLinkPartsError::with_tracing(
                    InitDbsProvidersLinkPartsErrorEnum::GetLocalProvidersLinkParts(*e),
                    where_was,
                ))),
                false => Err(Box::new(InitDbsProvidersLinkPartsError::new(
                    InitDbsProvidersLinkPartsErrorEnum::GetLocalProvidersLinkParts(*e),
                    where_was,
                ))),
            }
        }
        Ok(providers_json_local_data_hashmap) => {
            let providers_json_local_data_hashmap_clone = providers_json_local_data_hashmap.clone();
            let (mongo_insert_data_option_result, postgres_insert_data_option_result) = tokio::join!(
                async {
                    if CONFIG.is_mongo_initialization_enabled {
                        return Some(init_mongo(providers_json_local_data_hashmap, false).await);
                    }
                    None
                },
                async {
                    if CONFIG.is_postgres_initialization_enabled {
                        return Some(
                            init_postgres(providers_json_local_data_hashmap_clone, false).await,
                        );
                    }
                    None
                }
            );
            match (
                mongo_insert_data_option_result,
                postgres_insert_data_option_result,
            ) {
                (None, None) => (),
                (None, Some(pg_result)) => {
                    if let Err(e) = pg_result {
                        let where_was = WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        };
                        match should_trace {
                            true => {
                                return Err(Box::new(
                                    InitDbsProvidersLinkPartsError::with_tracing(
                                        InitDbsProvidersLinkPartsErrorEnum::PostgresInit(*e),
                                        where_was,
                                    ),
                                ));
                            }
                            false => {
                                return Err(Box::new(InitDbsProvidersLinkPartsError::new(
                                    InitDbsProvidersLinkPartsErrorEnum::PostgresInit(*e),
                                    where_was,
                                )));
                            }
                        }
                    }
                }
                (Some(mongo_result), None) => {
                    if let Err(e) = mongo_result {
                        let where_was = WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        };
                        match should_trace {
                            true => {
                                return Err(Box::new(
                                    InitDbsProvidersLinkPartsError::with_tracing(
                                        InitDbsProvidersLinkPartsErrorEnum::MongoInit(*e),
                                        where_was,
                                    ),
                                ));
                            }
                            false => {
                                return Err(Box::new(InitDbsProvidersLinkPartsError::new(
                                    InitDbsProvidersLinkPartsErrorEnum::MongoInit(*e),
                                    where_was,
                                )));
                            }
                        }
                    }
                }
                (Some(mongo_result), Some(pg_result)) => {
                    match (mongo_result, pg_result) {
                        (Ok(_), Ok(_)) => (),
                        (Ok(_), Err(e)) => {
                            let where_was = WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            };
                            match should_trace {
                                true => {
                                    return Err(Box::new(
                                        InitDbsProvidersLinkPartsError::with_tracing(
                                            InitDbsProvidersLinkPartsErrorEnum::PostgresInit(*e),
                                            where_was,
                                        ),
                                    ));
                                }
                                false => {
                                    return Err(Box::new(InitDbsProvidersLinkPartsError::new(
                                        InitDbsProvidersLinkPartsErrorEnum::PostgresInit(*e),
                                        where_was,
                                    )));
                                }
                            }
                        }
                        (Err(e), Ok(_)) => {
                            let where_was = WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            };
                            match should_trace {
                                true => {
                                    return Err(Box::new(
                                        InitDbsProvidersLinkPartsError::with_tracing(
                                            InitDbsProvidersLinkPartsErrorEnum::MongoInit(*e),
                                            where_was,
                                        ),
                                    ));
                                }
                                false => {
                                    return Err(Box::new(InitDbsProvidersLinkPartsError::new(
                                        InitDbsProvidersLinkPartsErrorEnum::MongoInit(*e),
                                        where_was,
                                    )));
                                }
                            }
                        }
                        (Err(mongo_error), Err(postgres_error)) => {
                            let where_was = WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            };
                            match should_trace {
                                true => {
                                    return Err(Box::new(InitDbsProvidersLinkPartsError::with_tracing(
                                    InitDbsProvidersLinkPartsErrorEnum::MongoAndPostgresInit {
                                        mongo: *mongo_error,
                                        postgres: *postgres_error,
                                    },
                                    where_was,
                                )));
                                }
                                false => {
                                    return Err(Box::new(InitDbsProvidersLinkPartsError::new(
                                        InitDbsProvidersLinkPartsErrorEnum::MongoAndPostgresInit {
                                            mongo: *mongo_error,
                                            postgres: *postgres_error,
                                        },
                                        where_was,
                                    )));
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }
    }
}
