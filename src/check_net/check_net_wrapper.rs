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
use crate::helpers::where_was::WhereWas;
use crate::traits::git_info_trait::GitInfo;

#[derive(Debug, GitInfoDerive)]
pub enum CheckNetWrapperErrorEnum {
    NetAndPostgresAndMongo {
        net: CheckNetAvailabilityErrorEnum,
        postgres: PostgresCheckAvailabilityError,
        mongo: MongoCheckAvailabilityError,
        where_was: WhereWas,
    },
    NetAndPostgres {
        net: CheckNetAvailabilityErrorEnum,
        postgres: PostgresCheckAvailabilityError,
        where_was: WhereWas,
    },
    NetAndMongo {
        net: CheckNetAvailabilityErrorEnum,
        mongo: MongoCheckAvailabilityError,
        where_was: WhereWas,
    },
    PostgresAndMongo {
        postgres: PostgresCheckAvailabilityError,
        mongo: MongoCheckAvailabilityError,
        where_was: WhereWas,
    },
    Net {
        source: CheckNetAvailabilityErrorEnum,
        where_was: WhereWas,
    },
    Postgres {
        source: PostgresCheckAvailabilityError,
        where_was: WhereWas,
    },
    Mongo {
        source: MongoCheckAvailabilityError,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
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
            Err(Box::new(CheckNetWrapperErrorEnum::NetAndPostgresAndMongo {
                net: *net_e,
                postgres: *postgres_e,
                mongo: mongo_e,
                where_was: WhereWas {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            }))
        }
        (Err(net_e), Err(postgres_e), Ok(_)) => {
            Err(Box::new(CheckNetWrapperErrorEnum::NetAndPostgres {
                net: *net_e,
                postgres: *postgres_e,
                where_was: WhereWas {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            }))
        }
        (Err(net_e), Ok(_), Err(mongo_e)) => Err(Box::new(CheckNetWrapperErrorEnum::NetAndMongo {
            net: *net_e,
            mongo: mongo_e,
            where_was: WhereWas {
                file: file!(),
                line: line!(),
                column: column!(),
            },
        })),
        (Ok(_), Err(postgres_e), Err(mongo_e)) => {
            Err(Box::new(CheckNetWrapperErrorEnum::PostgresAndMongo {
                postgres: *postgres_e,
                mongo: mongo_e,
                where_was: WhereWas {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            }))
        }
        (Err(e), Ok(_), Ok(_)) => Err(Box::new(CheckNetWrapperErrorEnum::Net {
            source: *e,
            where_was: WhereWas {
                file: file!(),
                line: line!(),
                column: column!(),
            },
        })),
        (Ok(_), Err(postgres_e), Ok(_)) => Err(Box::new(CheckNetWrapperErrorEnum::Postgres {
            source: *postgres_e,
            where_was: WhereWas {
                file: file!(),
                line: line!(),
                column: column!(),
            },
        })),
        (Ok(_), Ok(_), Err(e)) => Err(Box::new(CheckNetWrapperErrorEnum::Mongo {
            source: e,
            where_was: WhereWas {
                file: file!(),
                line: line!(),
                column: column!(),
            },
        })),
        (Ok(_), Ok(_), Ok(_)) => Ok(()),
    }
}
