use crate::check_net::check_link::check_link;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::mongo_integration::mongo_check_availability::mongo_check_availability;

// use crate::check_net::check_link_metainfo_structures::HandledReachProviderStatusInfo;
// use crate::check_net::check_link_metainfo_structures::UnhandledReachProviderInfo;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_wrapper() -> bool {
    let starting_link_checked = check_link(&CONFIG.params.starting_check_link).0;
    let mut postgres_link_checked = false;
    let result_postgres_link_checked = PgConnection::establish(&postgres_get_db_url());
    match result_postgres_link_checked {
        Ok(_) => {
            postgres_link_checked = true;
        }
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "server cannot reach {}, error: {:#?}",
                    &postgres_get_db_url(),
                    e
                ),
            );
            //todo
        }
    }
    let mut mongo_link_checked = false;
    let result_mongo_link_checked = mongo_check_availability(
        &mongo_get_db_url(),
        &CONFIG.mongo_params.providers_db_name_handle,
    );
    match result_mongo_link_checked {
        Ok(_) => {
            mongo_link_checked = true;
        }
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "server cannot reach {}, error: {:#?}",
                    &mongo_get_db_url(),
                    e
                ),
            );
            //todo
        }
    }
    starting_link_checked && postgres_link_checked && mongo_link_checked
}
