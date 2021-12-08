use crate::check_net::check_link_status_code::check_link_status_code;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::check_net::check_net_error_enum::CheckNetError;

use reqwest::StatusCode;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_link_availability(link: &str) -> Result<(), CheckNetError> {
    let status_code = check_link_status_code(&CONFIG.starting_check_link)?;
    if !StatusCode::is_success(&status_code) {
        return Err(CheckNetError::StartingLinkCode { status_code });
    }
    Ok(())
}
