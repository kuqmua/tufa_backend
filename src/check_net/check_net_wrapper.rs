use crate::check_net::check_link::check_link;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use diesel::pg::PgConnection;
use diesel::prelude::ConnectionError;
use diesel::prelude::*;

use crate::mongo_integration::mongo_check_availability::mongo_check_availability;

use crate::check_net::check_link_metainfo_structures::HandledReachProviderStatusInfo;
use crate::check_net::check_link_metainfo_structures::UnhandledReachProviderInfo;

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

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_wrapper() -> Result<(), CheckNetError> {
    let check_link_result = check_link(&CONFIG.params.starting_check_link);
    if !check_link_result.0 {
        return Err(CheckNetError::StartingLink {
            handled: check_link_result.2,
            unhandled: check_link_result.1,
        });
    }
    let result_postgres_link_checked: Result<PgConnection, ConnectionError> =
        PgConnection::establish(&postgres_get_db_url());
    match result_postgres_link_checked {
        Ok(_) => {}
        Err(e) => {
            return Err(CheckNetError::Postgres { error: e });
        }
    }
    let result_mongo_link_checked = mongo_check_availability(
        &mongo_get_db_url(),
        &CONFIG.mongo_params.providers_db_name_handle,
    );
    match result_mongo_link_checked {
        Ok(_) => {}
        Err(e) => {
            return Err(CheckNetError::Mongo { error: e });
        }
    }
    Ok(())
}
