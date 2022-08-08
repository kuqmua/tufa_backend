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

#[derive(Debug)] //, ErrorDisplay
pub struct CheckNetAvailabilityError {
    pub source: CheckNetAvailabilityErrorEnum,
    pub where_was: WhereWas,
}

impl fmt::Display for CheckNetAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => write!(f, "{}\n{}", self.source, self.where_was),
        }
    }
}

#[derive(Debug, GitInfoDerive)] //, ErrorDisplay
pub enum CheckNetAvailabilityErrorEnum {
    ReqwestGet(reqwest::Error),
    Client(StatusCode),
    Server(StatusCode),
}

impl fmt::Display for CheckNetAvailabilityErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => match self {
                CheckNetAvailabilityErrorEnum::ReqwestGet(e) => {
                    write!(f, "{}", e)
                }
                CheckNetAvailabilityErrorEnum::Client(e) => {
                    write!(f, "{}", e)
                }
                CheckNetAvailabilityErrorEnum::Server(e) => {
                    write!(f, "{}", e)
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
pub async fn check_net_availability(link: &str) -> Result<(), Box<CheckNetAvailabilityError>> {
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
            Err(Box::new(CheckNetAvailabilityError {
                source: CheckNetAvailabilityErrorEnum::ReqwestGet(e),
                where_was,
            }))
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
                return Err(Box::new(CheckNetAvailabilityError {
                    source: CheckNetAvailabilityErrorEnum::Client(status),
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
                return Err(Box::new(CheckNetAvailabilityError {
                    source: CheckNetAvailabilityErrorEnum::Server(status),
                    where_was,
                }));
            }
            Ok(())
        }
    }
}
