use crate::check_net::check_link_status_code::check_link_status_code;

use reqwest::StatusCode;

#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub enum CheckNetAvailabilityError {
    ///CheckNetAvailabilityError: reqwest: {0:?}
    ReqwestError(
        #[from]
        #[source]
        Box<reqwest::Error>,
    ),
    ///CheckNetAvailabilityError: StatusCode: {0:?}
    StatusCode(Box<StatusCode>),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_availability(link: &str) -> Result<(), Box<CheckNetAvailabilityError>> {
    match check_link_status_code(link) {
        Err(e) => Err(Box::new(CheckNetAvailabilityError::ReqwestError(e))),
        Ok(status_code) => {
            if !StatusCode::is_success(&status_code) {
                return Err(Box::new(CheckNetAvailabilityError::StatusCode(Box::new(
                    status_code,
                ))));
            }
            Ok(())
        }
    }
}
