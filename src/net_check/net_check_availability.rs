use crate::config_mods::lazy_static_config::CONFIG;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use git_info::GitInfo;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_get_source_for_parent_error_struct::ImplGetSourceForParentErrorStruct;
use impl_get_source_for_simple_error_enum::ImplGetSourceForSimpleErrorEnum;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
use init_error_with_tracing_for_original_error_struct::InitErrorWithTracingForOriginalErrorStruct;
use reqwest::StatusCode;
use tufa_common::traits::get_source::GetSource;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    ImplDisplayForErrorStruct,
    ImplGetSourceForParentErrorStruct,
    // ImplGetWhereWasForErrorStruct,
    InitError,
    InitErrorWithTracingForOriginalErrorStruct,
)]
pub struct NetCheckAvailabilityError {
    source: NetCheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

impl crate::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for NetCheckAvailabilityError {
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        tufa_common::where_was::WhereWasOneOrMany::One(
            tufa_common::where_was::WhereWasWithAddition {
                additional_info: None,
                where_was: self.where_was.clone(),
            },
        )
    }
}

#[derive(Debug, GitInfo, ImplDisplayForSimpleErrorEnum, ImplGetSourceForSimpleErrorEnum)]
pub enum NetCheckAvailabilityErrorEnum {
    ReqwestGet(reqwest::Error),
    Client(StatusCode),
    Server(StatusCode),
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
                    where_was,
                ))),
                false => Err(Box::new(NetCheckAvailabilityError::new(
                    NetCheckAvailabilityErrorEnum::ReqwestGet(e),
                    where_was,
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
                            where_was,
                        )));
                    }
                    false => {
                        return Err(Box::new(NetCheckAvailabilityError::new(
                            NetCheckAvailabilityErrorEnum::Client(status),
                            where_was,
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
                            where_was,
                        )));
                    }
                    false => {
                        return Err(Box::new(NetCheckAvailabilityError::new(
                            NetCheckAvailabilityErrorEnum::Server(status),
                            where_was,
                        )));
                    }
                }
            }
            Ok(())
        }
    }
}
