use crate::check_net::check_link_status_code::check_link_status_code;
use crate::check_net::check_link_status_code::CheckLinkStatusCodeError;

use crate::check_net::check_is_status_code_successfull::check_is_status_code_successfull;
use crate::check_net::check_is_status_code_successfull::StatusCodeError;

#[derive(Debug)]
pub enum CheckNetAvailabilityErrorEnum {
    CheckLinkStatusCodeError {
        source: CheckLinkStatusCodeError,
        line: String,
    },
    StatusCodeError {
        source: StatusCodeError,
        line: String,
    },
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn check_net_availability(link: &str) -> Result<(), Box<CheckNetAvailabilityErrorEnum>> {
    match check_link_status_code(link).await {
        Err(e) => Err(Box::new(
            CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError {
                source: e,
                line: format!("{}:{}:{}", file!(), line!(), column!()),
            },
        )),
        Ok(status_code) => {
            if let Err(e) = check_is_status_code_successfull(status_code) {
                return Err(Box::new(CheckNetAvailabilityErrorEnum::StatusCodeError {
                    source: e,
                    line: format!("{}:{}:{}", file!(), line!(), column!()),
                }));
            }
            Ok(())
        }
    }
}
