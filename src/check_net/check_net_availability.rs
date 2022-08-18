use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::git_info::GIT_INFO;
use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasTracing;
use crate::traits::git_info_trait::GitInfo;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use git_info::GitInfoDerive;
use reqwest::StatusCode;
use std::fmt;
// use struct_field_getter::DeriveStructFieldGetter;
// use struct_field_setter::DeriveStructFieldSetter;
use tracing::error;
extern crate chrono;
// use error_display::ErrorDisplay;
//, DeriveStructFieldGetter, DeriveStructFieldSetter
#[derive(Debug)] //, ErrorDisplay
pub struct CheckNetAvailabilityError {
    source: CheckNetAvailabilityErrorEnum,
    pub where_was: Vec<WhereWas>,
}

//

#[derive(Debug, Clone)]
pub struct TimeFileLineColumn {
    pub time: DateTime<FixedOffset>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl CheckNetAvailabilityError {
    pub fn new(source: CheckNetAvailabilityErrorEnum, where_was: Vec<WhereWas>) -> Self {
        if where_was.len() == 1 {
            if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled {
                error!(
                    error = format!("{}", source),
                    source = where_was[0].source_place(),
                    github_source = where_was[0].github_source_place(),
                );
            } else if CONFIG.is_show_source_place_enabled {
                error!(
                    error = format!("{}", source),
                    source = where_was[0].source_place(),
                );
            } else if CONFIG.is_show_github_source_place_enabled {
                error!(
                    error = format!("{}", source),
                    github_source = where_was[0].github_source_place(),
                );
            } else {
                error!(error = format!("{}", source),);
            }
        }
        Self { source, where_was }
    }
}
//

impl fmt::Display for CheckNetAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => write!(f, "{}\n{:?}", self.source, self.where_was),
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

//

#[derive(Debug, Clone)]
pub enum CheckNetAvailabilityErrorTracing {
    Message(String),
    Child(Vec<WhereWas>),
}

impl CheckNetAvailabilityError {
    // pub fn new(
    //     source: CheckNetAvailabilityErrorEnum,
    //     where_was: WhereWas,
    //     option_child_or_message: Option<WhereWasTracing>,
    // ) -> Self {
    //     let s = Self { source, where_was };
    //     if let Some(child_or_message) = option_child_or_message {
    //         // s.tracing_trace(child_or_message);
    //         // s.tracing_debug(child_or_message);
    //         // s.tracing_info(child_or_message);
    //         // s.tracing_warn(child_or_message);
    //         s.tracing_error(child_or_message);
    //     }
    //     s
    // }
    // fn tracing_error(&self, child_or_error: WhereWasTracing) {
    //     //impl std::error::Error
    //     match child_or_error {
    //         WhereWasTracing::Message(e) => {
    //             if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled
    //             {
    //                 error!(
    //                     error = format!("{}", e),
    //                     source = self.where_was.source_place(),
    //                     github_source = self.where_was.github_source_place(),
    //                 );
    //             } else if CONFIG.is_show_source_place_enabled {
    //                 error!(
    //                     error = format!("{}", e),
    //                     source = self.where_was.source_place(),
    //                 );
    //             } else if CONFIG.is_show_github_source_place_enabled {
    //                 error!(
    //                     error = format!("{}", e),
    //                     github_source = self.where_was.github_source_place(),
    //                 );
    //             } else {
    //                 error!(error = format!("{}", e),);
    //             }
    //         }
    //         WhereWasTracing::Child(c) => {
    //             if CONFIG.is_parent_tracing_enabled {
    //                 let child_sources = c.iter().enumerate().fold(
    //                     String::from(""),
    //                     |mut acc, (index, where_was)| {
    //                         let elem = format!(" {} {}", index, where_was.source_place());
    //                         acc.push_str(&elem);
    //                         acc
    //                     },
    //                 );
    //                 let github_sources = c.iter().enumerate().fold(
    //                     String::from(""),
    //                     |mut acc, (index, where_was)| {
    //                         let elem = format!(" {} {}", index, where_was.github_source_place());
    //                         acc.push_str(&elem);
    //                         acc
    //                     },
    //                 );
    //                 if CONFIG.is_show_source_place_enabled
    //                     && CONFIG.is_show_github_source_place_enabled
    //                 {
    //                     error!(
    //                         source = self.where_was.source_place(),
    //                         github_source = self.where_was.github_source_place(),
    //                         child_sources = child_sources,
    //                         child_github_sources = github_sources,
    //                     );
    //                 } else if CONFIG.is_show_source_place_enabled {
    //                     error!(
    //                         source = self.where_was.source_place(),
    //                         child_sources = child_sources,
    //                     );
    //                 } else if CONFIG.is_show_github_source_place_enabled {
    //                     error!(
    //                         github_source = self.where_was.github_source_place(),
    //                         child_github_sources = github_sources,
    //                     );
    //                 } else {
    //                     error!(source = String::from("disabled"));
    //                 }
    //             }
    //         }
    //     }
    // }
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
            Err(Box::new(CheckNetAvailabilityError {
                source: CheckNetAvailabilityErrorEnum::ReqwestGet(e),
                where_was: vec![where_was],
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
                return Err(Box::new(CheckNetAvailabilityError {
                    source: CheckNetAvailabilityErrorEnum::Client(status),
                    where_was: vec![where_was],
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
                // return Err(Box::new(CheckNetAvailabilityError::new(
                //     CheckNetAvailabilityErrorEnum::Server(status),
                //     where_was: vec![where_was],
                // )));
                // Ok(())
                return Err(Box::new(CheckNetAvailabilityError {
                    source: CheckNetAvailabilityErrorEnum::Server(status),
                    where_was: vec![where_was],
                }));
            }
            Ok(())
        }
    }
}
