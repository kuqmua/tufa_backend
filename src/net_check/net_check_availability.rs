use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::git_info::GIT_INFO;
use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasOneOrFew;
use crate::traits::git_info_trait::GitInfo;
use crate::traits::tufa_error::TufaError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use git_info::GitInfoDerive;
use init_error::DeriveInitError;
use reqwest::StatusCode;
use std::fmt;
extern crate chrono;
// use init_error_with_tracing::DeriveInitErrorWithTracing;
//DeriveInitErrorWithTracing,
#[derive(Debug, DeriveInitError)]
pub struct NetCheckAvailabilityError {
    source: NetCheckAvailabilityErrorEnum,
    where_was: Vec<WhereWasOneOrFew>,
}

impl NetCheckAvailabilityError {
    pub fn with_tracing(
        source: NetCheckAvailabilityErrorEnum,
        where_was: Vec<crate::helpers::where_was::WhereWasOneOrFew>,
    ) -> Self {
        if where_was.len() == 1 {
            if let Some(first_value) = where_was.get(0) {
                match first_value {
                    crate::helpers::where_was::WhereWasOneOrFew::One(where_was_one) => {
                        //todo different formating for source impl
                        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
                            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                                tracing::error!(
                                    error = format!("{}", source),
                                    // children_source = format!("{}", &self.get_where_was()),
                                    source = where_was_one.source_place(),
                                );
                            }
                            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                                tracing::error!(
                                    error = format!("{}", source),
                                    // error = format!("{}", &self.get_source()),
                                    // children_source = format!("{}", &self.get_where_was()),
                                    github_source = where_was_one.github_source_place(),
                                );
                            }
                            crate::config_mods::source_place_type::SourcePlaceType::None => {
                                tracing::error!(error = format!("{}", source));
                            }
                        }
                    }
                    crate::helpers::where_was::WhereWasOneOrFew::Few(hs_where_was) => {
                        tracing::error!(error = "todo WhereWasOneOrFew::Few",)
                    }
                }
            }
            //todo next elements
        }
        Self { source, where_was }
    }
}

impl TufaError for NetCheckAvailabilityError {
    fn get_source(&self) -> String {
        format!("{}", self.source)
    }
    fn get_where_was(&self) -> String {
        match CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self.where_was),
            false => {
                let mut content =
                    self.where_was
                        .clone()
                        .iter()
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&format!("{},", elem));
                            acc
                        });
                if !content.is_empty() {
                    content.pop();
                }
                content
            }
        }
    }
}

impl fmt::Display for NetCheckAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => write!(f, "{}\n{:#?}", self.source, self.where_was),
        }
    }
}

#[derive(Debug, GitInfoDerive)]
pub enum NetCheckAvailabilityErrorEnum {
    ReqwestGet(reqwest::Error),
    Client(StatusCode),
    Server(StatusCode),
}

impl fmt::Display for NetCheckAvailabilityErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => match self {
                NetCheckAvailabilityErrorEnum::ReqwestGet(e) => {
                    write!(f, "{}", e)
                }
                NetCheckAvailabilityErrorEnum::Client(e) => {
                    write!(f, "{}", e)
                }
                NetCheckAvailabilityErrorEnum::Server(e) => {
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
pub async fn net_check_availability(
    link: &str,
    should_trace: bool,
) -> Result<(), Box<NetCheckAvailabilityError>> {
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
                true => Err(Box::new(NetCheckAvailabilityError::with_tracing(
                    NetCheckAvailabilityErrorEnum::ReqwestGet(e),
                    vec![WhereWasOneOrFew::One(where_was)],
                ))),
                false => Err(Box::new(NetCheckAvailabilityError::new(
                    NetCheckAvailabilityErrorEnum::ReqwestGet(e),
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
                        return Err(Box::new(NetCheckAvailabilityError::with_tracing(
                            NetCheckAvailabilityErrorEnum::Client(status),
                            vec![WhereWasOneOrFew::One(where_was)],
                        )));
                    }
                    false => {
                        return Err(Box::new(NetCheckAvailabilityError::new(
                            NetCheckAvailabilityErrorEnum::Client(status),
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
                        return Err(Box::new(NetCheckAvailabilityError::with_tracing(
                            NetCheckAvailabilityErrorEnum::Server(status),
                            vec![WhereWasOneOrFew::One(where_was)],
                        )));
                    }
                    false => {
                        return Err(Box::new(NetCheckAvailabilityError::new(
                            NetCheckAvailabilityErrorEnum::Server(status),
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
