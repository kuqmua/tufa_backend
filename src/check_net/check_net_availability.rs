use std::fmt;

use chrono::{DateTime, FixedOffset, Local, Utc};

use crate::check_net::check_status_code::check_status_code;
use crate::check_net::check_status_code::CheckStatusCodeError;

use crate::config_mods::lazy_static_config::CONFIG;
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

impl fmt::Display for CheckNetAvailabilityErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
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
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(3 * 3600)),
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
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(3 * 3600)),
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
