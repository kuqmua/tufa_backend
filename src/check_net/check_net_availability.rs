use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::git_info::GIT_INFO;
use crate::helpers::where_was::WhereWas;
use crate::traits::git_info_trait::GitInfo;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use git_info::GitInfoDerive;
use reqwest::StatusCode;
use std::fmt;
// use error_display::ErrorDisplay;

#[derive(Debug, GitInfoDerive)] //, ErrorDisplay
pub enum CheckNetAvailabilityErrorEnum {
    CheckLinkStatusCode {
        source: reqwest::Error,
        where_was: WhereWas,
    },
    Client {
        source: StatusCode,
        where_was: WhereWas,
    },
    Server {
        source: StatusCode,
        where_was: WhereWas,
    },
}

impl fmt::Display for CheckNetAvailabilityErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => match self {
                CheckNetAvailabilityErrorEnum::CheckLinkStatusCode { source, where_was } => {
                    write!(f, "{}\n{}", source, where_was)
                }
                CheckNetAvailabilityErrorEnum::Client { source, where_was } => {
                    write!(f, "{}\n{}", source, where_was)
                }
                CheckNetAvailabilityErrorEnum::Server { source, where_was } => {
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
            where_was.tracing_error(format!("{}", e));
            Err(Box::new(
                CheckNetAvailabilityErrorEnum::CheckLinkStatusCode {
                    source: e,
                    where_was,
                },
            ))
        }
        Ok(res) => {
            let status = res.status();
            if status.is_client_error() {
                let where_was = WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                };
                where_was.tracing_error(format!("check net client error: {}", status));
                return Err(Box::new(CheckNetAvailabilityErrorEnum::Client {
                    source: status,
                    where_was,
                }));
            }
            if status.is_server_error() {
                let where_was = WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                };
                where_was.tracing_error(format!("check net server error: {}", status));
                return Err(Box::new(CheckNetAvailabilityErrorEnum::Server {
                    source: status,
                    where_was,
                }));
            }
            Ok(())
        }
    }
}
