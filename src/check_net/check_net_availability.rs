use crate::check_net::check_link_status_code::check_link_status_code;
use crate::check_net::check_link_status_code::CheckLinkStatusCodeError;

use crate::check_net::check_status_code::check_status_code;
use crate::check_net::check_status_code::StatusCodeError;

use crate::helpers::get_git_commit_string::get_git_commit_string;
use crate::traits::git_info_trait::GitInfo;

#[derive(Debug, GitInfoDerive)]
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
            if let Err(e) = check_status_code(status_code) {
                return Err(Box::new(CheckNetAvailabilityErrorEnum::StatusCodeError {
                    source: *e,
                    line: format!("{}:{}:{}", file!(), line!(), column!()),
                }));
            }
            Ok(())
        }
    }
}
