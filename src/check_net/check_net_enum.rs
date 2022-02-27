use std::fmt;

use chrono::{DateTime, FixedOffset, Local, Utc};
use strum_macros::EnumIter;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::helpers::where_was::WhereWas;
use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::check_net::check_net_availability::check_net_availability;
use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::postgres_integration::postgres_check_availability::postgres_check_availability;

use crate::check_net::check_net_availability::CheckNetAvailabilityErrorEnum;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorEnum;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;

#[derive(Debug, EnumIter)]
pub enum CheckNet {
    Net,
    Postgres,
    Mongo,
}

#[derive(Debug)]
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

impl fmt::Display for CheckNetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled {
            write!(
                f,
                "{}\n{}\n{}",
                self.where_was.source_place_with_readable_time(),
                self.where_was.github_source_place_with_readable_time(),
                self.source
            )
        } else if CONFIG.is_show_source_place_enabled {
            write!(
                f,
                "{}\n{}",
                self.where_was.source_place_with_readable_time(),
                self.source
            )
        } else if CONFIG.is_show_github_source_place_enabled {
            write!(
                f,
                "{}\n{}",
                self.where_was.github_source_place_with_readable_time(),
                self.source
            )
        } else {
            write!(f, "{}", self.source)
        }
    }
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
            CheckNet::Mongo => postgres_get_db_url(),
            CheckNet::Postgres => mongo_get_db_url(),
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
                                .with_timezone(&FixedOffset::east(3 * 3600)),
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
                                .with_timezone(&FixedOffset::east(3 * 3600)),
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
                                .with_timezone(&FixedOffset::east(3 * 3600)),
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
