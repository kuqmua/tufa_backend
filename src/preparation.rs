use crate::check_net::check_net_availability::check_net_availability;
use crate::check_net::check_net_availability::CheckNetAvailabilityError;
use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::mongo::get_mongo_url::get_mongo_url;
use crate::helpers::postgres::get_postgres_url::get_postgres_url;
use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasTracing;
use crate::init_dbs_logic::init_dbs::init_dbs;
use crate::init_dbs_logic::init_tables_enum::InitTablesEnumError;
use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityError;
use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::join;
use std::fmt::Display;

#[derive(Debug)]
pub enum PreparationErrorEnum {
    Net {
        source: Box<CheckNetAvailabilityError>,
        where_was: WhereWas,
    },
    Postgres {
        source: Box<PostgresCheckAvailabilityError>,
        where_was: WhereWas,
    },
    Mongo {
        source: Box<MongoCheckAvailabilityError>,
        where_was: WhereWas,
    },
    NetAndMongo {
        net_source: Box<CheckNetAvailabilityError>,
        mongo_source: Box<MongoCheckAvailabilityError>,
        where_was: WhereWas,
    },
    NetAndPostgres {
        net_source: Box<CheckNetAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
        where_was: WhereWas,
    },
    MongoAndPostgres {
        mongo_source: Box<MongoCheckAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
        where_was: WhereWas,
    },
    NetAndMongoAndPostgres {
        net_source: Box<CheckNetAvailabilityError>,
        mongo_source: Box<MongoCheckAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
        where_was: WhereWas,
    },
    InitDbs {
        source: Vec<Box<InitTablesEnumError>>,
        where_was: WhereWas,
    },
}

impl Display for PreparationErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => match self {
                PreparationErrorEnum::Net { source, where_was } => {
                    write!(f, "{}\n{}", *source, where_was)
                }
                PreparationErrorEnum::Postgres { source, where_was } => {
                    write!(f, "{}\n{}", *source, where_was)
                }
                PreparationErrorEnum::Mongo { source, where_was } => {
                    write!(f, "{}\n{}", *source, where_was)
                }
                PreparationErrorEnum::NetAndMongo {
                    net_source,
                    mongo_source,
                    where_was,
                } => {
                    write!(f, "{}\n{}\n{}", net_source, mongo_source, where_was)
                }
                PreparationErrorEnum::NetAndPostgres {
                    net_source,
                    postgres_source,
                    where_was,
                } => {
                    write!(f, "{}\n{}\n{}", net_source, postgres_source, where_was)
                }
                PreparationErrorEnum::MongoAndPostgres {
                    mongo_source,
                    postgres_source,
                    where_was,
                } => {
                    write!(f, "{}\n{}\n{}", mongo_source, postgres_source, where_was)
                }
                PreparationErrorEnum::NetAndMongoAndPostgres {
                    net_source,
                    mongo_source,
                    postgres_source,
                    where_was,
                } => {
                    write!(
                        f,
                        "{}\n{}\n{}\n{}",
                        net_source, mongo_source, postgres_source, where_was
                    )
                }
                PreparationErrorEnum::InitDbs { source, where_was } => {
                    write!(f, "{:#?}\n{}", source, where_was)
                }
            },
        }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn preparation() -> Result<(), Box<PreparationErrorEnum>> {
    let net_url = &CONFIG.starting_check_link.clone();
    let postgres_url = &get_postgres_url();
    let mongo_url = &get_mongo_url();
    match join!(
        check_net_availability(net_url),
        postgres_check_availability(postgres_url),
        mongo_check_availability(mongo_url),
    ) {
        (Ok(_), Ok(_), Ok(_)) => (),
        (Ok(_), Ok(_), Err(m)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            where_was.tracing_error(WhereWasTracing::Child(vec![m.where_was.clone()]));
            return Err(Box::new(PreparationErrorEnum::Mongo {
                source: m,
                where_was,
            }));
        }
        (Ok(_), Err(p), Ok(_)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            where_was.tracing_error(WhereWasTracing::Child(vec![p.where_was.clone()]));
            return Err(Box::new(PreparationErrorEnum::Postgres {
                source: p,
                where_was,
            }));
        }
        (Ok(_), Err(p), Err(m)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            where_was.tracing_error(WhereWasTracing::Child(vec![m.where_was.clone()]));
            return Err(Box::new(PreparationErrorEnum::MongoAndPostgres {
                mongo_source: m,
                postgres_source: p,
                where_was,
            }));
        }
        (Err(n), Ok(_), Ok(_)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            where_was.tracing_error(WhereWasTracing::Child(vec![n.where_was.clone()]));
            return Err(Box::new(PreparationErrorEnum::Net {
                source: n,
                where_was,
            }));
        }
        (Err(n), Ok(_), Err(m)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            where_was.tracing_error(WhereWasTracing::Child(vec![
                n.where_was.clone(),
                m.where_was.clone(),
            ]));
            return Err(Box::new(PreparationErrorEnum::NetAndMongo {
                net_source: n,
                mongo_source: m,
                where_was,
            }));
        }
        (Err(n), Err(p), Ok(_)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            where_was.tracing_error(WhereWasTracing::Child(vec![
                n.where_was.clone(),
                p.where_was.clone(),
            ]));
            return Err(Box::new(PreparationErrorEnum::NetAndPostgres {
                net_source: n,
                postgres_source: p,
                where_was,
            }));
        }
        (Err(n), Err(p), Err(m)) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            where_was.tracing_error(WhereWasTracing::Child(vec![
                n.where_was.clone(),
                p.where_was.clone(),
                m.where_was.clone(),
            ]));
            return Err(Box::new(PreparationErrorEnum::NetAndMongoAndPostgres {
                net_source: n,
                postgres_source: p,
                mongo_source: m,
                where_was,
            }));
        }
    }
    //todo: add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
    if !CONFIG.is_dbs_initialization_enabled
        || (!CONFIG.is_mongo_initialization_enabled && !CONFIG.is_postgres_initialization_enabled)
    {
        return Ok(());
    }
    if let Err(e) = init_dbs().await {
        return Err(Box::new(PreparationErrorEnum::InitDbs {
            source: e,
            where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        }));
    }
    Ok(())
}
