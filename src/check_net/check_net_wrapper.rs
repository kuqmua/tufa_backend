use crate::check_net::check_link::check_link;

use crate::config_mods::config::CONFIG;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use diesel::pg::PgConnection;
use diesel::prelude::ConnectionError;
use diesel::prelude::*;

use crate::mongo_integration::mongo_check_availability::mongo_check_availability;

use crate::check_net::check_link_metainfo_structures::HandledReachProviderStatusInfo;
use crate::check_net::check_link_metainfo_structures::UnhandledReachProviderInfo;

#[derive(Debug)]
pub enum CheckNetError {
    StartingLink {
        handled: HandledReachProviderStatusInfo,
        unhandled: UnhandledReachProviderInfo,
    },
    Postgres {
        error: ConnectionError,
    },
    Mongo {
        error: mongodb::error::Error,
    },
}
impl From<mongodb::error::Error> for CheckNetError {
    fn from(e: mongodb::error::Error) -> Self {
        CheckNetError::Mongo { error: e }
    }
}
impl From<ConnectionError> for CheckNetError {
    fn from(e: ConnectionError) -> Self {
        CheckNetError::Postgres { error: e }
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_wrapper() -> Result<(), CheckNetError> {
    let check_link_result = check_link(&CONFIG.params.starting_check_link);
    if !check_link_result.0 {
        return Err(CheckNetError::StartingLink {
            handled: check_link_result.2,
            unhandled: check_link_result.1,
        });
    }
    PgConnection::establish(&postgres_get_db_url())?;
    mongo_check_availability(&mongo_get_db_url())?;
    Ok(())
}
