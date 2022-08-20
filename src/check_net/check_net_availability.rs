use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::git_info::GIT_INFO;
use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasOneOrFew;
use crate::traits::git_info_trait::GitInfo;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use git_info::GitInfoDerive;
use init_error::DeriveInitError;
use init_error_with_tracing::DeriveInitErrorWithTracing;
use reqwest::StatusCode;
use std::fmt;
extern crate chrono;

#[derive(Debug, DeriveInitErrorWithTracing, DeriveInitError)]
pub struct CheckNetAvailabilityError {
    source: CheckNetAvailabilityErrorEnum,
    where_was: Vec<WhereWasOneOrFew>,
}

impl fmt::Display for CheckNetAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => write!(f, "{}\n{:#?}", self.source, self.where_was),
        }
    }
}

#[derive(Debug, GitInfoDerive)]
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
pub async fn check_net_availability(
    link: &str,
    should_trace: bool,
) -> Result<(), Box<CheckNetAvailabilityError>> {
    match reqwest::get(link).await {
        Err(e) => {
            let where_was = WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            };
            match should_trace {
                true => Err(Box::new(CheckNetAvailabilityError::with_tracing(
                    CheckNetAvailabilityErrorEnum::ReqwestGet(e),
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
                false => Err(Box::new(CheckNetAvailabilityError::new(
                    CheckNetAvailabilityErrorEnum::ReqwestGet(e),
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
            }
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
                match should_trace {
                    true => {
                        return Err(Box::new(CheckNetAvailabilityError::with_tracing(
                            CheckNetAvailabilityErrorEnum::Client(status),
                            vec![WhereWasOneOrFew::One(where_was)],
                        )));
                    }
                    false => {
                        return Err(Box::new(CheckNetAvailabilityError::new(
                            CheckNetAvailabilityErrorEnum::Client(status),
                            vec![WhereWasOneOrFew::One(where_was)],
                        )));
                    }
                }
            }
            if status.is_server_error() {
                let where_was = WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                };
                match should_trace {
                    true => {
                        return Err(Box::new(CheckNetAvailabilityError::with_tracing(
                            CheckNetAvailabilityErrorEnum::Server(status),
                            vec![WhereWasOneOrFew::One(where_was)],
                        )));
                    }
                    false => {
                        return Err(Box::new(CheckNetAvailabilityError::new(
                            CheckNetAvailabilityErrorEnum::Server(status),
                            vec![WhereWasOneOrFew::One(where_was)],
                        )));
                    }
                }
            }
            Ok(())
        }
    }
}

// pub fn one() -> Result<(), SomeError> {
//     if let Err(e) = library_function_one() {
//         tracing::error!(error = e);
//         return Err(e)
//     }
//     Ok(())
// }
// pub fn two() -> Result<(), AnotherError> {
//     if let Err(e) = one() {
//         tracing::error!(error = e);
//         return Err(AnotherError::One(e))
//     }
//     if let Err(e) = library_function_two() {
//         tracing::error!(error = e);
//         return Err(AnotherError::Two(e));
//     }
//     Ok(())
// }
