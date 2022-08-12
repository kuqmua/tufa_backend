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

#[derive(Debug)] //, ErrorDisplay
pub struct PreparationError {
    pub source: PreparationErrorEnum,
    pub where_was: WhereWas,
}

#[derive(Debug)]
pub enum PreparationErrorEnum {
    Net(CheckNetAvailabilityError),
    Postgres(PostgresCheckAvailabilityError),
    Mongo(MongoCheckAvailabilityError),
    NetAndMongo {
        net_source: Box<CheckNetAvailabilityError>,
        mongo_source: Box<MongoCheckAvailabilityError>,
    },
    NetAndPostgres {
        net_source: Box<CheckNetAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
    },
    MongoAndPostgres {
        mongo_source: Box<MongoCheckAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
    },
    NetAndMongoAndPostgres {
        net_source: Box<CheckNetAvailabilityError>,
        mongo_source: Box<MongoCheckAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
    },
    InitDbs(Vec<Box<InitTablesEnumError>>),
}

impl Display for PreparationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => write!(f, "{} {}", self.where_was, self.source),
        }
    }
}

impl Display for PreparationErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => match self {
                PreparationErrorEnum::Net(source) => {
                    write!(f, "{}", *source)
                }
                PreparationErrorEnum::Postgres(source) => {
                    write!(f, "{}", *source)
                }
                PreparationErrorEnum::Mongo(source) => {
                    write!(f, "{}", *source)
                }
                PreparationErrorEnum::NetAndMongo {
                    net_source,
                    mongo_source,
                } => {
                    write!(f, "{}\n{}", net_source, mongo_source)
                }
                PreparationErrorEnum::NetAndPostgres {
                    net_source,
                    postgres_source,
                } => {
                    write!(f, "{}\n{}", net_source, postgres_source)
                }
                PreparationErrorEnum::MongoAndPostgres {
                    mongo_source,
                    postgres_source,
                } => {
                    write!(f, "{}\n{}", mongo_source, postgres_source)
                }
                PreparationErrorEnum::NetAndMongoAndPostgres {
                    net_source,
                    mongo_source,
                    postgres_source,
                } => {
                    write!(f, "{}\n{}\n{}", net_source, mongo_source, postgres_source)
                }
                PreparationErrorEnum::InitDbs(source) => {
                    write!(f, "{:#?}", *source)
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
pub async fn preparation() -> Result<(), Box<PreparationError>> {
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
            let where_was = WhereWas::new(
                DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file!(),
                line!(),
                column!(),
                Some(WhereWasTracing::Child(vec![m.where_was.clone()])),
            );
            return Err(Box::new(PreparationError {
                source: PreparationErrorEnum::Mongo(*m),
                where_was,
            }));
        }
        (Ok(_), Err(p), Ok(_)) => {
            let where_was = WhereWas::new(
                DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file!(),
                line!(),
                column!(),
                Some(WhereWasTracing::Child(vec![p.where_was.clone()])),
            );
            return Err(Box::new(PreparationError {
                source: PreparationErrorEnum::Postgres(*p),
                where_was,
            }));
        }
        (Ok(_), Err(p), Err(m)) => {
            let where_was = WhereWas::new(
                DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file!(),
                line!(),
                column!(),
                Some(WhereWasTracing::Child(vec![m.where_was.clone()])),
            );
            return Err(Box::new(PreparationError {
                source: PreparationErrorEnum::MongoAndPostgres {
                    mongo_source: m,
                    postgres_source: p,
                },
                where_was,
            }));
        }
        (Err(n), Ok(_), Ok(_)) => {
            let where_was = WhereWas::new(
                DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file!(),
                line!(),
                column!(),
                Some(WhereWasTracing::Child(vec![n.where_was.clone()])),
            );
            return Err(Box::new(PreparationError {
                source: PreparationErrorEnum::Net(*n),
                where_was,
            }));
        }
        (Err(n), Ok(_), Err(m)) => {
            let where_was = WhereWas::new(
                DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file!(),
                line!(),
                column!(),
                Some(WhereWasTracing::Child(vec![
                    n.where_was.clone(),
                    m.where_was.clone(),
                ])),
            );
            return Err(Box::new(PreparationError {
                source: PreparationErrorEnum::NetAndMongo {
                    net_source: n,
                    mongo_source: m,
                },
                where_was,
            }));
        }
        (Err(n), Err(p), Ok(_)) => {
            let where_was = WhereWas::new(
                DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file!(),
                line!(),
                column!(),
                Some(WhereWasTracing::Child(vec![
                    n.where_was.clone(),
                    p.where_was.clone(),
                ])),
            );
            return Err(Box::new(PreparationError {
                source: PreparationErrorEnum::NetAndPostgres {
                    net_source: n,
                    postgres_source: p,
                },
                where_was,
            }));
        }
        (Err(n), Err(p), Err(m)) => {
            let where_was = WhereWas::new(
                DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file!(),
                line!(),
                column!(),
                Some(WhereWasTracing::Child(vec![
                    n.where_was.clone(),
                    p.where_was.clone(),
                    m.where_was.clone(),
                ])),
            );
            return Err(Box::new(PreparationError {
                source: PreparationErrorEnum::NetAndMongoAndPostgres {
                    net_source: n,
                    postgres_source: p,
                    mongo_source: m,
                },
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
        let where_was = WhereWas::new(
            DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file!(),
            line!(),
            column!(),
            None,
        );
        return Err(Box::new(PreparationError {
            source: PreparationErrorEnum::InitDbs(e),
            where_was,
        }));
    }
    Ok(())
}
