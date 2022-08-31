use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::mongo::get_mongo_url::get_mongo_url;
use crate::helpers::postgres::get_postgres_url::get_postgres_url;
use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasOneOrFew;
use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityError;
use crate::net_check::net_check_availability::net_check_availability;
use crate::net_check::net_check_availability::NetCheckAvailabilityError;
use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use crate::traits::get_source::GetSource;
use crate::traits::get_where_was::GetWhereWas;
use crate::traits::tufa_error::TufaError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::join;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_get_source_for_simple_error_enum::ImplGetSourceForSimpleErrorEnum;
use impl_get_where_was_for_enum::ImplGetWhereWasForEnum;
use init_error::DeriveInitError;
use std::fmt::Display;
// use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
// use crate::helpers::where_was;
// // use init_error_with_tracing::DeriveInitErrorWithTracing;

//DeriveInitErrorWithTracing
#[derive(Debug, DeriveInitError)] //, ImplDisplayForErrorStruct
pub struct CheckAvailabilityError {
    source: CheckAvailabilityErrorEnum,
    where_was: Vec<WhereWasOneOrFew>,
}

impl TufaError for CheckAvailabilityError {
    fn get_source(&self) -> String {
        self.source.get_source()
    }
    fn get_where_was(&self) -> String {
        match CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self.where_was),
            false => {
                let mut content =
                    self.where_was
                        .clone()
                        .iter()
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&format!("{},", elem));
                            acc
                        });
                if !content.is_empty() {
                    content.pop();
                }
                let child_where_was = self.source.get_where_was();
                match child_where_was.is_empty() {
                    true => content,
                    false => format!("{}, {}", content, child_where_was),
                }
            }
        }
    }
}

impl CheckAvailabilityError {
    pub fn with_tracing(
        source: CheckAvailabilityErrorEnum,
        where_was: Vec<crate::helpers::where_was::WhereWasOneOrFew>,
    ) -> Self {
        if where_was.len() == 1 {
            if let Some(first_value) = where_was.get(0) {
                match first_value {
                    crate::helpers::where_was::WhereWasOneOrFew::One(where_was_one) => {
                        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
                            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                                tracing::error!(
                                    error = format!("{}", source.get_source()),
                                    children_source = format!("{}", source.get_where_was()),
                                    source_place = where_was_one.source_place(),
                                );
                            }
                            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                                tracing::error!(
                                    error = format!("{}", source.get_source()),
                                    children_source = format!("{}", source.get_where_was()),
                                    github_source_place = where_was_one.github_source_place(),
                                );
                            }
                            crate::config_mods::source_place_type::SourcePlaceType::None => {
                                tracing::error!(error = format!("{}", source));
                            }
                        }
                    }
                    crate::helpers::where_was::WhereWasOneOrFew::Few(hs_where_was) => {
                        tracing::error!(error = "todo WhereWasOneOrFew::Few",)
                    }
                }
            }
            //todo next elements
        }
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

impl Display for CheckAvailabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => write!(f, "{:?} {}", self.where_was, self.source),
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
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::Mongo(*m),
                    vec![WhereWasOneOrFew::One(where_was)],
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
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::Postgres(*p),
                    vec![WhereWasOneOrFew::One(where_was)],
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
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::MongoAndPostgres {
                        mongo_source: m,
                        postgres_source: p,
                    },
                    vec![WhereWasOneOrFew::One(where_was)],
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
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::Net(*n),
                    vec![WhereWasOneOrFew::One(where_was)],
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
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::NetAndMongo {
                        net_source: n,
                        mongo_source: m,
                    },
                    vec![WhereWasOneOrFew::One(where_was)],
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
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
                false => Err(Box::new(CheckAvailabilityError::new(
                    CheckAvailabilityErrorEnum::NetAndPostgres {
                        net_source: n,
                        postgres_source: p,
                    },
                    vec![WhereWasOneOrFew::One(where_was)],
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
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
                false => Err(Box::new(CheckAvailabilityError::with_tracing(
                    CheckAvailabilityErrorEnum::NetAndMongoAndPostgres {
                        net_source: n,
                        postgres_source: p,
                        mongo_source: m,
                    },
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
            }
        }
    }
}
