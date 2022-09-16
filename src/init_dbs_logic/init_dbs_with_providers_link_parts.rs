use crate::init_dbs_logic::init_mongo::init_mongo;
use crate::init_dbs_logic::init_mongo::InitMongoError;
use crate::init_dbs_logic::init_postgres::init_postgres;
use crate::init_dbs_logic::init_postgres::PostgresInitError;
use crate::lazy_static::config::CONFIG;
use crate::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use crate::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany;
use tufa_common::traits::with_tracing::WithTracing;
use tufa_common::where_was::WhereWas;
use tufa_common::where_was::WhereWasOneOrMany;
// use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;

#[derive(Debug, InitError)] //, ImplGetWhereWasForErrorStruct
pub struct InitDbsProvidersLinkPartsError {
    source: InitDbsProvidersLinkPartsErrorEnum,
    where_was: WhereWas,
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
    for InitDbsProvidersLinkPartsError
{
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        let mut vec = Vec::new();
        self.source
            .get_where_was_one_or_many()
            .into_vec()
            .into_iter()
            .for_each(|w| {
                vec.push(w);
            });
        vec.push(tufa_common::where_was::WhereWasWithAddition {
            additional_info: None,
            where_was: self.where_was.clone(),
        });
        tufa_common::where_was::WhereWasOneOrMany::Many(vec)
    }
}

// impl crate::traits::get_where_was_one_or_many::GetWhereWas for InitDbsProvidersLinkPartsError {
//     fn get_where_was(&self) -> String {
//         match crate::lazy_static::config::CONFIG.is_debug_implementation_enable {
//             true => format!("{:#?} {:#?}", self.where_was, self.source.get_where_was()),
//             false => format!("{} {}", self.where_was, self.source.get_where_was()),
//         }
//     }
// }

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

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
    for InitDbsProvidersLinkPartsErrorEnum
{
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        match self {
            InitDbsProvidersLinkPartsErrorEnum::GetLocalProvidersLinkParts(e) => {
                e.get_where_was_one_or_many()
            }
            InitDbsProvidersLinkPartsErrorEnum::PostgresInit(e) => e.get_where_was_one_or_many(),
            InitDbsProvidersLinkPartsErrorEnum::MongoInit(e) => e.get_where_was_one_or_many(),
            InitDbsProvidersLinkPartsErrorEnum::MongoAndPostgresInit { mongo, postgres } => {
                let mut vec = Vec::new();
                mongo
                    .get_where_was_one_or_many()
                    .into_vec()
                    .into_iter()
                    .for_each(|w| {
                        vec.push(w);
                    });
                postgres
                    .get_where_was_one_or_many()
                    .into_vec()
                    .into_iter()
                    .for_each(|w| {
                        vec.push(w);
                    });
                tufa_common::where_was::WhereWasOneOrMany::Many(vec)
            }
        }
    }
}

// impl crate::traits::get_where_was_one_or_many::GetWhereWas for InitDbsProvidersLinkPartsErrorEnum {
//     fn get_where_was(&self) -> String {
//         match self {
//             InitDbsProvidersLinkPartsErrorEnum::GetLocalProvidersLinkParts(e) => e.get_where_was(),
//             InitDbsProvidersLinkPartsErrorEnum::PostgresInit(e) => e.get_where_was(),
//             InitDbsProvidersLinkPartsErrorEnum::MongoInit(e) => e.get_where_was(),
//             InitDbsProvidersLinkPartsErrorEnum::MongoAndPostgresInit { mongo, postgres } => {
//                 format!(
//                     "[ {}, {} ]",
//                     mongo.get_where_was(),
//                     postgres.get_where_was()
//                 )
//             }
//         }
//     }
// }

impl tufa_common::traits::get_source::GetSource for InitDbsProvidersLinkPartsErrorEnum {
    fn get_source(&self) -> String {
        match crate::lazy_static::config::CONFIG.is_debug_implementation_enable {
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

impl tufa_common::traits::with_tracing::WithTracing<InitDbsProvidersLinkPartsErrorEnum>
    for InitDbsProvidersLinkPartsError
{
    fn with_tracing(source: InitDbsProvidersLinkPartsErrorEnum, where_was: WhereWas) -> Self {
        match crate::lazy_static::config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = source.get_source(),
                    source_place = where_was.file_line_column(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = source.get_source(),
                    github_source_place = where_was
                        .github_file_line_column(&crate::lazy_static::git_info::GIT_INFO.data),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = source.get_source());
            }
        }
        Self { source, where_was }
    }
}

impl tufa_common::traits::get_source::GetSource for InitDbsProvidersLinkPartsError {
    fn get_source(&self) -> String {
        match crate::lazy_static::config::CONFIG.is_debug_implementation_enable {
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
