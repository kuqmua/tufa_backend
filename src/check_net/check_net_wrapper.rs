use crate::check_net::check_link_status_code::check_link_status_code;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::check_net::check_net_error_enum::CheckNetError;

use reqwest::StatusCode;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_wrapper() -> Result<(), CheckNetError> {
    let status_code = check_link_status_code(&CONFIG.params.starting_check_link)?;
    if !StatusCode::is_success(&status_code) {
        return Err(CheckNetError::StartingLinkCode { status_code });
    }
    postgres_check_availability(&postgres_get_db_url())?;
    mongo_check_availability(&mongo_get_db_url())?;
    Ok(())
}
