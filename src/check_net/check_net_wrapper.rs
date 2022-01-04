use crate::config_mods::lazy_static_config::CONFIG;

use crate::check_net::check_net_availability::check_net_availability;
use crate::check_net::check_net_error_enum::CheckNetError;

use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_wrapper() -> Result<(), Box<CheckNetError>> {
    //todo to it in parallel?
    check_net_availability(&CONFIG.starting_check_link)?;
    postgres_check_availability(&postgres_get_db_url())?;
    mongo_check_availability(&mongo_get_db_url())?;
    Ok(())
}
