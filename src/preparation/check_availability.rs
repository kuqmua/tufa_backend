use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityError;
use crate::net_check::net_check_availability::net_check_availability;
use crate::net_check::net_check_availability::NetCheckAvailabilityError;
use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::join;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_get_source_for_parent_error_struct::ImplGetSourceForParentErrorStruct;
use impl_get_source_for_simple_error_enum::ImplGetSourceForSimpleErrorEnum;
use impl_get_where_was_for_enum::ImplGetWhereWasForEnum;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
use init_error_with_tracing::InitErrorWithTracing;
use tufa_common::traits::get_bunyan_where_was::GetBunyanWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany;
use tufa_common::traits::with_tracing::WithTracing;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    // ImplGetWhereWasForErrorStruct,
    ImplGetSourceForParentErrorStruct,
    ImplDisplayForErrorStruct,
    InitError,
    // InitErrorWithTracing,
)]
pub struct CheckAvailabilityError {
    source: CheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

impl tufa_common::traits::with_tracing::WithTracing<CheckAvailabilityErrorEnum>
    for CheckAvailabilityError
{
    fn with_tracing(
        source: CheckAvailabilityErrorEnum,
        where_was: tufa_common::where_was::WhereWas,
    ) -> Self {
        match crate::lazy_static::config::CONFIG.source_place_type {
            tufa_common::config::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = format!("{}", source.get_source()),
                    where_was = format!(
                        "{} {}",
                        where_was.file_line_column(),
                        source.get_bunyan_where_was(&CONFIG.source_place_type, &GIT_INFO.data),
                    ),
                );
            }
            tufa_common::config::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = format!("{}", source.get_source()),
                    where_was = format!(
                        "{} {}",
                        where_was
                            .github_file_line_column(&crate::lazy_static::git_info::GIT_INFO.data),
                        source.get_bunyan_where_was(&CONFIG.source_place_type, &GIT_INFO.data)
                    ),
                );
            }
            tufa_common::config::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = format!("{}", source));
            }
        }
        Self { source, where_was }
    }
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
    for CheckAvailabilityError
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

// impl crate::traits::get_where_was_one_or_many::GetWhereWas for CheckAvailabilityError {
//     fn get_where_was(&self) -> String {
//         match crate::lazy_static::config::CONFIG.is_debug_implementation_enable {
//             true => format!("{:#?} {:#?}", self.where_was, self.source.get_where_was()),
//             false => format!("{} {}", self.where_was, self.source.get_where_was()),
//         }
//     }
// }

#[derive(
    Debug,
    ImplGetSourceForSimpleErrorEnum,
    ImplDisplayForSimpleErrorEnum, //ImplGetWhereWasForEnum,
)]
pub enum CheckAvailabilityErrorEnum {
    Net(Box<NetCheckAvailabilityError>),
    Postgres(Box<PostgresCheckAvailabilityError>),
    Mongo(Box<MongoCheckAvailabilityError>),
    NetAndMongo {
        net_source: Box<NetCheckAvailabilityError>,
        mongo_source: Box<MongoCheckAvailabilityError>,
    },
    NetAndPostgres {
        net_source: Box<NetCheckAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
    },
    MongoAndPostgres {
        mongo_source: Box<MongoCheckAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
    },
    NetAndMongoAndPostgres {
        net_source: Box<NetCheckAvailabilityError>,
        mongo_source: Box<MongoCheckAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
    },
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany
    for CheckAvailabilityErrorEnum
{
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        match self {
            CheckAvailabilityErrorEnum::Net(e) => e.get_where_was_one_or_many(),
            CheckAvailabilityErrorEnum::Postgres(e) => e.get_where_was_one_or_many(),
            CheckAvailabilityErrorEnum::Mongo(e) => e.get_where_was_one_or_many(),
            CheckAvailabilityErrorEnum::NetAndMongo {
                net_source,
                mongo_source,
            } => {
                let mut vec = Vec::<tufa_common::where_was::WhereWasWithAddition>::new();
                net_source
                    .get_where_was_one_or_many()
                    .into_vec()
                    .into_iter()
                    .for_each(|w| {
                        vec.push(w);
                    });
                mongo_source
                    .get_where_was_one_or_many()
                    .into_vec()
                    .into_iter()
                    .for_each(|w| {
                        vec.push(w);
                    });
                tufa_common::where_was::WhereWasOneOrMany::Many(vec)
            }
            CheckAvailabilityErrorEnum::NetAndPostgres {
                net_source,
                postgres_source,
            } => {
                let mut vec = Vec::<tufa_common::where_was::WhereWasWithAddition>::new();
                net_source
                    .get_where_was_one_or_many()
                    .into_vec()
                    .into_iter()
                    .for_each(|w| {
                        vec.push(w);
                    });
                postgres_source
                    .get_where_was_one_or_many()
                    .into_vec()
                    .into_iter()
                    .for_each(|w| {
                        vec.push(w);
                    });
                tufa_common::where_was::WhereWasOneOrMany::Many(vec)
            }
            CheckAvailabilityErrorEnum::MongoAndPostgres {
                mongo_source,
                postgres_source,
            } => {
                let mut vec = Vec::<tufa_common::where_was::WhereWasWithAddition>::new();
                mongo_source
                    .get_where_was_one_or_many()
                    .into_vec()
                    .into_iter()
                    .for_each(|w| {
                        vec.push(w);
                    });
                postgres_source
                    .get_where_was_one_or_many()
                    .into_vec()
                    .into_iter()
                    .for_each(|w| {
                        vec.push(w);
                    });
                tufa_common::where_was::WhereWasOneOrMany::Many(vec)
            }
            CheckAvailabilityErrorEnum::NetAndMongoAndPostgres {
                net_source,
                mongo_source,
                postgres_source,
            } => {
                let mut vec = Vec::<tufa_common::where_was::WhereWasWithAddition>::new();
                net_source
                    .get_where_was_one_or_many()
                    .into_vec()
                    .into_iter()
                    .for_each(|w| {
                        vec.push(w);
                    });
                mongo_source
                    .get_where_was_one_or_many()
                    .into_vec()
                    .into_iter()
                    .for_each(|w| {
                        vec.push(w);
                    });
                postgres_source
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

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn check_availability(should_trace: bool) -> Result<(), Box<CheckAvailabilityError>> {
    let net_url = &CONFIG.starting_check_link.clone();
    let postgres_url = &CONFIG.get_postgres_url();
    let mongo_url = &CONFIG.get_mongo_url();
    match join!(
        net_check_availability(net_url, false),
        postgres_check_availability(postgres_url, false),
        mongo_check_availability(mongo_url, false),
    ) {
        (Ok(_), Ok(_), Ok(_)) => Ok(()),
        (Ok(_), Ok(_), Err(m)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(CheckAvailabilityError::with_tracing(
                    CheckAvailabilityErrorEnum::Mongo(m),
                    where_was,
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::Mongo(m),
                    where_was,
                ))),
            }
        }
        (Ok(_), Err(p), Ok(_)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(CheckAvailabilityError::with_tracing(
                    CheckAvailabilityErrorEnum::Postgres(p),
                    where_was,
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::Postgres(p),
                    where_was,
                ))),
            }
        }
        (Ok(_), Err(p), Err(m)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(CheckAvailabilityError::with_tracing(
                    CheckAvailabilityErrorEnum::MongoAndPostgres {
                        mongo_source: m,
                        postgres_source: p,
                    },
                    where_was,
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::MongoAndPostgres {
                        mongo_source: m,
                        postgres_source: p,
                    },
                    where_was,
                ))),
            }
        }
        (Err(n), Ok(_), Ok(_)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(CheckAvailabilityError::with_tracing(
                    CheckAvailabilityErrorEnum::Net(n),
                    where_was,
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::Net(n),
                    where_was,
                ))),
            }
        }
        (Err(n), Ok(_), Err(m)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(CheckAvailabilityError::with_tracing(
                    CheckAvailabilityErrorEnum::NetAndMongo {
                        net_source: n,
                        mongo_source: m,
                    },
                    where_was,
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::NetAndMongo {
                        net_source: n,
                        mongo_source: m,
                    },
                    where_was,
                ))),
            }
        }
        (Err(n), Err(p), Ok(_)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(CheckAvailabilityError::with_tracing(
                    CheckAvailabilityErrorEnum::NetAndPostgres {
                        net_source: n,
                        postgres_source: p,
                    },
                    where_was,
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::NetAndPostgres {
                        net_source: n,
                        postgres_source: p,
                    },
                    where_was,
                ))),
            }
        }
        (Err(n), Err(p), Err(m)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(CheckAvailabilityError::with_tracing(
                    CheckAvailabilityErrorEnum::NetAndMongoAndPostgres {
                        net_source: n,
                        postgres_source: p,
                        mongo_source: m,
                    },
                    where_was,
                ))),
                false => Err(Box::new(CheckAvailabilityError::with_tracing(
                    CheckAvailabilityErrorEnum::NetAndMongoAndPostgres {
                        net_source: n,
                        postgres_source: p,
                        mongo_source: m,
                    },
                    where_was,
                ))),
            }
        }
    }
}
