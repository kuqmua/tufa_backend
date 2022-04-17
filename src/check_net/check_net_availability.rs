use crate::check_net::check_status_code::check_status_code;
use crate::check_net::check_status_code::CheckStatusCodeError;
use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::traits::git_info_trait::GitInfo;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use error_display::ErrorDisplay;
use git_info::GitInfoDerive;
use std::fmt;
use tufa_common::helpers::git::get_git_commit_string::get_git_commit_string;

#[derive(Debug, GitInfoDerive, ErrorDisplay)]
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
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
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
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
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
