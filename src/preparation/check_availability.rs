use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::mongo::get_mongo_url::get_mongo_url;
use crate::helpers::postgres::get_postgres_url::get_postgres_url;
use crate::helpers::where_was::WhereWas;
use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityError;
use crate::net_check::net_check_availability::net_check_availability;
use crate::net_check::net_check_availability::NetCheckAvailabilityError;
use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use crate::traits::get_source::GetSource;
use crate::traits::get_where_was::GetWhereWas;
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

#[derive(
    Debug,
    ImplGetWhereWasForErrorStruct,
    ImplGetSourceForParentErrorStruct,
    ImplDisplayForErrorStruct,
)] //DeriveInitError, DeriveInitErrorWithTracing
pub struct CheckAvailabilityError {
    source: CheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

impl CheckAvailabilityError {
    pub fn with_tracing(source: CheckAvailabilityErrorEnum, where_was: WhereWas) -> Self {
        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = format!("{}", source.get_source()),
                    children_source = format!("{}", source.get_where_was()),
                    source_place = where_was.source_place(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = format!("{}", source.get_source()),
                    children_source = format!("{}", source.get_where_was()),
                    github_source_place = where_was.github_source_place(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = format!("{}", source));
            }
        }
        Self { source, where_was }
    }
    pub fn new(source: CheckAvailabilityErrorEnum, where_was: WhereWas) -> Self {
        Self { source, where_was }
    }
}

#[derive(
    Debug, ImplGetSourceForSimpleErrorEnum, ImplGetWhereWasForEnum, ImplDisplayForSimpleErrorEnum,
)]
pub enum CheckAvailabilityErrorEnum {
    Net(NetCheckAvailabilityError),
    Postgres(PostgresCheckAvailabilityError),
    Mongo(MongoCheckAvailabilityError),
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

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn check_availability(should_trace: bool) -> Result<(), Box<CheckAvailabilityError>> {
    let net_url = &CONFIG.starting_check_link.clone();
    let postgres_url = &get_postgres_url();
    let mongo_url = &get_mongo_url();
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
                    CheckAvailabilityErrorEnum::Mongo(*m),
                    where_was,
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::Mongo(*m),
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
                    CheckAvailabilityErrorEnum::Postgres(*p),
                    where_was,
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::Postgres(*p),
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
                    CheckAvailabilityErrorEnum::Net(*n),
                    where_was,
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::Net(*n),
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
