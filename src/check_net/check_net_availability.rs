use crate::check_net::check_link_status_code::check_link_status_code;
use crate::check_net::check_net_error_enum::CheckNetError;

use reqwest::StatusCode;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_availability(link: &str) -> Result<(), CheckNetError> {
    let status_code = check_link_status_code(link)?;
    if !StatusCode::is_success(&status_code) {
        return Err(CheckNetError::StartingLinkCode { status_code });
    }
    Ok(())
}
