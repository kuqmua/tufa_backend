// use crate::check_net::check_net_wrapper::{check_net_wrapper, CheckNetWrapperError};
use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::init_dbs_logic::init_dbs::init_dbs;
use crate::init_dbs_logic::init_tables_enum::InitTablesEnumError;
// use chrono::{DateTime, FixedOffset, Local, Utc};
use std::fmt::Display;

use crate::check_net::check_net_availability::check_net_availability;
use crate::check_net::check_net_availability::CheckNetAvailabilityErrorEnum;
use crate::helpers::mongo::get_mongo_url::get_mongo_url;
use crate::helpers::postgres::get_postgres_url::get_postgres_url;
use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorEnum;
use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;

#[derive(Debug)]
pub enum PreparationErrorEnum {
    CheckNet {
        source: Box<CheckNetAvailabilityErrorEnum>,
        where_was: WhereWas,
    },
    Postgres {
        source: Box<PostgresCheckAvailabilityError>,
        where_was: WhereWas,
    },
    Mongo {
        source: Box<MongoCheckAvailabilityErrorEnum>,
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
                PreparationErrorEnum::CheckNet { source, where_was } => {
                    write!(f, "{}\n{}", *source, where_was)
                }
                PreparationErrorEnum::Postgres { source, where_was } => {
                    write!(f, "{}\n{}", *source, where_was)
                }
                PreparationErrorEnum::Mongo { source, where_was } => {
                    write!(f, "{}\n{}", *source, where_was)
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
    if let Err(e) = check_net_availability(&CONFIG.starting_check_link.clone()).await {
        return Err(Box::new(PreparationErrorEnum::CheckNet {
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
    if let Err(e) = postgres_check_availability(&get_postgres_url()).await {
        return Err(Box::new(PreparationErrorEnum::Postgres {
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
    if let Err(e) = mongo_check_availability(&get_mongo_url()).await {
        return Err(Box::new(PreparationErrorEnum::Mongo {
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
