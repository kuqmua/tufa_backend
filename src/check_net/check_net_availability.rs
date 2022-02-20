use crate::check_net::check_status_code::check_status_code;
use crate::check_net::check_status_code::CheckStatusCodeError;

use crate::helpers::get_git_commit_string::get_git_commit_string;
use crate::traits::git_info_trait::GitInfo;

use crate::helpers::where_was::WhereWas;

#[derive(Debug, GitInfoDerive)]
pub enum CheckNetAvailabilityErrorEnum {
    CheckLinkStatusCodeError {
        source: reqwest::Error,
        where_was: WhereWas,
    },
    StatusCodeError {
        source: CheckStatusCodeError,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn check_net_availability(link: &str) -> Result<(), Box<CheckNetAvailabilityErrorEnum>> {
    match reqwest::get(link).await {
        Err(e) => Err(Box::new(
            CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError {
                source: e,
                where_was: WhereWas {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            },
        )),
        Ok(res) => {
            if let Err(e) = check_status_code(res.status()) {
                return Err(Box::new(CheckNetAvailabilityErrorEnum::StatusCodeError {
                    source: *e,
                    where_was: WhereWas {
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                }));
            }
            Ok(())
        }
    }
}
