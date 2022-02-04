use crate::check_net::check_status_code::check_status_code;
use crate::check_net::check_status_code::CheckStatusCodeError;

use crate::helpers::get_git_commit_string::get_git_commit_string;
use crate::traits::git_info_trait::GitInfo;

#[derive(Debug, GitInfoDerive)]
pub enum CheckNetAvailabilityErrorEnum {
    CheckLinkStatusCodeError {
        source: reqwest::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    StatusCodeError {
        source: CheckStatusCodeError,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn check_net_availability(link: &str) -> Result<(), Box<CheckNetAvailabilityErrorEnum>> {
    match reqwest::get(link).await {
        Err(e) => Err(Box::new(
            CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError {
                source: e,
                file: file!(),
                line: line!(),
                column: column!(),
            },
        )),
        Ok(res) => {
            if let Err(e) = check_status_code(res.status()) {
                return Err(Box::new(CheckNetAvailabilityErrorEnum::StatusCodeError {
                    source: *e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                }));
            }
            Ok(())
        }
    }
}
