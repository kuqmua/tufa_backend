use crate::check_net::check_status_code::check_status_code;
use crate::check_net::check_status_code::CheckStatusCodeError;
use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::git_info::GIT_INFO;
use crate::helpers::where_was::WhereWas;
use crate::traits::git_info_trait::GitInfo;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use error_display::ErrorDisplay;
use git_info::GitInfoDerive;
use std::fmt;
use tracing::error;

#[derive(Debug, GitInfoDerive)] //, ErrorDisplay
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
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => match self {
                CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError { source, where_was } => {
                    write!(f, "{}\n{}", source, where_was)
                }
                CheckNetAvailabilityErrorEnum::StatusCodeError { source, where_was } => {
                    write!(f, "{}\n{}", source, where_was)
                }
            },
        }
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
        Err(e) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            error!(
                error = format!("{}", e),
                where_was = format!("{}", where_was)
            );
            Err(Box::new(
                CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError {
                    source: e,
                    where_was: where_was,
                },
            ))
        }
        Ok(res) => {
            if let Err(e) = check_status_code(res.status()) {
                let where_was = WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                };
                error!(
                    error = format!("{}", e),
                    where_was = format!("{}", where_was)
                );
                return Err(Box::new(CheckNetAvailabilityErrorEnum::StatusCodeError {
                    source: *e,
                    where_was: where_was,
                }));
            }
            println!("okkkk");
            Ok(())
        }
    }
}
