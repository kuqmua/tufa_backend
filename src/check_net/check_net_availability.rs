use crate::check_net::check_link_status_code::check_link_status_code;
use crate::check_net::check_link_status_code::CheckLinkStatusCodeError;

use crate::check_net::check_is_status_code_successfull::check_is_status_code_successfull;
use crate::check_net::check_is_status_code_successfull::StatusCodeError;

#[derive(displaydoc::Display, Debug, BoxErrFromErrDerive)]
pub struct CheckNetAvailabilityError {
    pub source: Box<CheckNetAvailabilityErrorEnum>,
}

impl From<CheckLinkStatusCodeError> for CheckNetAvailabilityError {
    fn from(error: CheckLinkStatusCodeError) -> Self {
        CheckNetAvailabilityError {
            source: Box::new(CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError(
                error,
            )),
        }
    }
}

impl From<StatusCodeError> for CheckNetAvailabilityError {
    fn from(error: StatusCodeError) -> Self {
        CheckNetAvailabilityError {
            source: Box::new(CheckNetAvailabilityErrorEnum::StatusCodeError(error)),
        }
    }
}

#[derive(Debug)]
pub enum CheckNetAvailabilityErrorEnum {
    CheckLinkStatusCodeError(CheckLinkStatusCodeError),
    StatusCodeError(StatusCodeError),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_availability(link: &str) -> Result<(), CheckNetAvailabilityError> {
    let status_code = check_link_status_code(link)?;
    check_is_status_code_successfull(status_code)?;
    Ok(())
}
