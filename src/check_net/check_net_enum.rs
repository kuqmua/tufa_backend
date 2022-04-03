use crate::check_net::check_net_availability::check_net_availability;
use crate::check_net::check_net_availability::CheckNetAvailabilityErrorEnum;
use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorEnum;
use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;
use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;
use chrono::{DateTime, FixedOffset, Local, Utc};
use std::fmt;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum CheckNet {
    Net,
    Postgres,
    Mongo,
}

#[derive(Debug, ErrorDisplay)]
pub struct CheckNetError {
    source: Box<CheckNetErrorEnum>,
    where_was: WhereWas,
}

#[derive(Debug)]
pub enum CheckNetErrorEnum {
    Net(CheckNetAvailabilityErrorEnum),
    Postgres(PostgresCheckAvailabilityError),
    Mongo(MongoCheckAvailabilityErrorEnum),
}

impl fmt::Display for CheckNetErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckNetErrorEnum::Net(e) => write!(f, "{e}"),
            CheckNetErrorEnum::Postgres(e) => write!(f, "{e}"),
            CheckNetErrorEnum::Mongo(e) => write!(f, "{e}"),
        }
    }
}

impl CheckNet {
    pub fn get_url(&self) -> String {
        match self {
            CheckNet::Net => CONFIG.starting_check_link.clone(),
            CheckNet::Mongo => mongo_get_db_url(),
            CheckNet::Postgres => postgres_get_db_url(),
        }
    }
    pub async fn check(self) -> Result<(), CheckNetError> {
        match self {
            CheckNet::Net => {
                if let Err(e) = check_net_availability(&self.get_url()).await {
                    return Err(CheckNetError {
                        source: Box::new(CheckNetErrorEnum::Net(*e)),
                        where_was: WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    });
                }
            }
            CheckNet::Postgres => {
                if let Err(e) = postgres_check_availability(&self.get_url()).await {
                    return Err(CheckNetError {
                        source: Box::new(CheckNetErrorEnum::Postgres(*e)),
                        where_was: WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    });
                }
            }
            CheckNet::Mongo => {
                if let Err(e) = mongo_check_availability(&self.get_url()).await {
                    return Err(CheckNetError {
                        source: Box::new(CheckNetErrorEnum::Mongo(*e)),
                        where_was: WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    });
                }
            }
        }
        Ok(())
    }
}
