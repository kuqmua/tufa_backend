use std::fmt;

use crate::check_net::check_link_status_code::check_link_status_code;

use reqwest::StatusCode;

#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub enum CheckNetAvailabilityError {
    ///CheckNetAvailabilityError: reqwest: {source:?}
    ReqwestError { source: Box<reqwest::Error> },
    ///CheckNetAvailabilityError: StatusCode: {source:?}
    StatusCode { source: StatusCodeError },
}

#[derive(Debug)]
pub struct StatusCodeError {
    pub status_code: StatusCode,
}

impl std::error::Error for StatusCodeError {}

impl fmt::Display for StatusCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StatusCodeError error")
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_availability(link: &str) -> Result<(), Box<CheckNetAvailabilityError>> {
    match check_link_status_code(link) {
        Err(e) => Err(Box::new(CheckNetAvailabilityError::ReqwestError {
            source: e,
        })),
        Ok(status_code) => {
            if !StatusCode::is_success(&status_code) {
                return Err(Box::new(CheckNetAvailabilityError::StatusCode {
                    source: StatusCodeError { status_code },
                }));
            }
            Ok(())
        }
    }
}
