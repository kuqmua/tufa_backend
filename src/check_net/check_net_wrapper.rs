use crate::config_mods::lazy_static_config::CONFIG;

use crate::check_net::check_net_availability::check_net_availability;
use crate::check_net::check_net_availability::CheckNetAvailabilityErrorEnum;

use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityError;
use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::helpers::get_git_commit_string::get_git_commit_string;
use crate::traits::git_info_trait::GitInfo;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, GitInfoDerive)]
pub enum CheckNetWrapperErrorEnum {
    NetAndPostgresAndMongo {
        net: CheckNetAvailabilityErrorEnum,
        postgres: PostgresCheckAvailabilityError,
        mongo: MongoCheckAvailabilityError,
        line: String,
    },
    NetAndPostgres {
        net: CheckNetAvailabilityErrorEnum,
        postgres: PostgresCheckAvailabilityError,
        line: String,
    },
    NetAndMongo {
        net: CheckNetAvailabilityErrorEnum,
        mongo: MongoCheckAvailabilityError,
        line: String,
    },
    PostgresAndMongo {
        postgres: PostgresCheckAvailabilityError,
        mongo: MongoCheckAvailabilityError,
        line: String,
    },
    Net {
        source: CheckNetAvailabilityErrorEnum,
        line: String,
    },
    Postgres {
        source: PostgresCheckAvailabilityError,
        line: String,
    },
    Mongo {
        source: MongoCheckAvailabilityError,
        line: String,
    },
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn check_net_wrapper() -> Result<(), Box<CheckNetWrapperErrorEnum>> {
    let starting_url = &CONFIG.starting_check_link;
    let postgres_url = &postgres_get_db_url();
    let mongo_url = &mongo_get_db_url();
    let result = tokio::join!(
        check_net_availability(starting_url),
        postgres_check_availability(postgres_url),
        mongo_check_availability(mongo_url),
    );
    match result {
        (Err(net_e), Err(postgres_e), Err(mongo_e)) => {
            return Err(Box::new(CheckNetWrapperErrorEnum::NetAndPostgresAndMongo {
                net: *net_e,
                postgres: postgres_e,
                mongo: mongo_e,
                line: format!("{}:{}:{}", file!(), line!(), column!()),
            }))
        }
        (Err(net_e), Err(postgres_e), Ok(_)) => {
            return Err(Box::new(CheckNetWrapperErrorEnum::NetAndPostgres {
                net: *net_e,
                postgres: postgres_e,
                line: format!("{}:{}:{}", file!(), line!(), column!()),
            }))
        }
        (Err(net_e), Ok(_), Err(mongo_e)) => {
            return Err(Box::new(CheckNetWrapperErrorEnum::NetAndMongo {
                net: *net_e,
                mongo: mongo_e,
                line: format!("{}:{}:{}", file!(), line!(), column!()),
            }));
        }
        (Ok(_), Err(postgres_e), Err(mongo_e)) => {
            return Err(Box::new(CheckNetWrapperErrorEnum::PostgresAndMongo {
                postgres: postgres_e,
                mongo: mongo_e,
                line: format!("{}:{}:{}", file!(), line!(), column!()),
            }));
        }
        (Err(e), Ok(_), Ok(_)) => {
            return Err(Box::new(CheckNetWrapperErrorEnum::Net {
                source: *e,
                line: format!("{}:{}:{}", file!(), line!(), column!()),
            }));
        }
        (Ok(_), Err(e), Ok(_)) => {
            return Err(Box::new(CheckNetWrapperErrorEnum::Postgres {
                source: e,
                line: format!("{}:{}:{}", file!(), line!(), column!()),
            }));
        }
        (Ok(_), Ok(_), Err(e)) => {
            return Err(Box::new(CheckNetWrapperErrorEnum::Mongo {
                source: e,
                line: format!("{}:{}:{}", file!(), line!(), column!()),
            }));
        }
        (Ok(_), Ok(_), Ok(_)) => Ok(()),
    }
}
