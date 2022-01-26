use std::fmt;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::check_net::check_net_availability::check_net_availability;
use crate::check_net::check_net_availability::CheckNetAvailabilityError;

use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityError;
use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

#[derive(Debug)]
pub struct CheckNetWrapperError {
    pub source: Box<CheckNetWrapperErrorEnum>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, ImplDisplayDerive)]
pub enum CheckNetWrapperErrorEnum {
    NetAndPostgresAndMongo {
        net: CheckNetAvailabilityError,
        postgres: PostgresCheckAvailabilityError,
        mongo: MongoCheckAvailabilityError,
    },
    NetAndPostgres {
        net: CheckNetAvailabilityError,
        postgres: PostgresCheckAvailabilityError,
    },
    NetAndMongo {
        net: CheckNetAvailabilityError,
        mongo: MongoCheckAvailabilityError,
    },
    PostgresAndMongo {
        postgres: PostgresCheckAvailabilityError,
        mongo: MongoCheckAvailabilityError,
    },
    Net(CheckNetAvailabilityError),
    Postgres(PostgresCheckAvailabilityError),
    Mongo(MongoCheckAvailabilityError),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn check_net_wrapper() -> Result<(), CheckNetWrapperError> {
    let starting_url = &CONFIG.starting_check_link;
    let postgres_url = &postgres_get_db_url();
    let mongo_url = &mongo_get_db_url();
    let result = tokio::join!(
        check_net_availability(starting_url),
        postgres_check_availability(postgres_url),
        mongo_check_availability(mongo_url),
    );
    match result {
        (Err(net_e), Err(postgres_e), Err(mongo_e)) => return Err(CheckNetWrapperError{
            source: Box::new(CheckNetWrapperErrorEnum::NetAndPostgresAndMongo{
                net: net_e,
                postgres: postgres_e,
                mongo: mongo_e,
            })
        }),
        (Err(net_e), Err(postgres_e), Ok(_)) => return Err(CheckNetWrapperError{
            source: Box::new(CheckNetWrapperErrorEnum::NetAndPostgres{
                net: net_e,
                postgres: postgres_e,
            })
        }),
        (Err(net_e), Ok(_), Err(mongo_e)) => return Err(CheckNetWrapperError{
            source: Box::new(CheckNetWrapperErrorEnum::NetAndMongo{
                net: net_e,
                mongo: mongo_e,
            })
        }),
        (Ok(_), Err(postgres_e), Err(mongo_e)) => return Err(CheckNetWrapperError{
            source: Box::new(CheckNetWrapperErrorEnum::PostgresAndMongo{
                postgres: postgres_e,
                mongo: mongo_e,
            })
        }),
        (Err(e), Ok(_), Ok(_)) => return Err(CheckNetWrapperError{
            source: Box::new(CheckNetWrapperErrorEnum::Net(e))
        }),
        (Ok(_), Err(e), Ok(_)) => return Err(CheckNetWrapperError{
            source: Box::new(CheckNetWrapperErrorEnum::Postgres(e))
        }),
        (Ok(_), Ok(_), Err(e)) => return Err(CheckNetWrapperError{
            source: Box::new(CheckNetWrapperErrorEnum::Mongo(e))
        }),
        (Ok(_), Ok(_), Ok(_)) => Ok(()),
    }
}
