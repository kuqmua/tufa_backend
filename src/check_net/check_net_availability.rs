use std::fmt;

use crate::check_net::check_link_status_code::check_link_status_code;
use crate::check_net::check_link_status_code::CheckLinkStatusCodeError;

use crate::check_net::check_is_status_code_successfull::check_is_status_code_successfull;
use crate::check_net::check_is_status_code_successfull::StatusCodeError;

#[derive(displaydoc::Display, Debug, BoxErrFromErrDerive)]
pub struct NetCheckAvailabilityError(Box<CheckNetAvailabilityError>);

#[derive(Debug)]
enum CheckNetAvailabilityError {
    ReqwestError(CheckLinkStatusCodeError),
    StatusCode(StatusCodeError),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_availability(link: &str) -> Result<(), NetCheckAvailabilityError> {
    match check_link_status_code(link) {
        Err(e) => Err(NetCheckAvailabilityError(Box::new(
            CheckNetAvailabilityError::ReqwestError(e),
        ))),
        Ok(status_code) => match check_is_status_code_successfull(status_code) {
            Err(e) => Err(NetCheckAvailabilityError(Box::new(
                CheckNetAvailabilityError::StatusCode(e),
            ))),
            Ok(_) => Ok(()),
        },
    }
}
