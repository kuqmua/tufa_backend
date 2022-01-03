use crate::check_net::check_link_status_code::check_link_status_code;
use crate::check_net::check_net_error_enum::CheckNetError;

use reqwest::StatusCode;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_availability(link: &str) -> Result<(), Box<CheckNetError>> {
    match check_link_status_code(link) {
        Err(e) => Err(Box::new(CheckNetError::ReqwestError(e))),
        Ok(status_code) => {
            if !StatusCode::is_success(&status_code) {
                return Err(Box::new(CheckNetError::StartingLinkCode(Box::new(
                    status_code,
                ))));
            }
            Ok(())
        }
    }
}
