use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use impl_display_for_error::ImplDisplayForError;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use reqwest::Error;
use reqwest::StatusCode;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_methods::WhereWasMethods;

#[derive(
    Debug,
    ImplDisplayForError,
    ImplGetSourceFromTufaCommon,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingFromTufaCommon,
)]
pub struct NetCheckAvailabilityWrapperError {
    source: NetCheckAvailabilityOriginErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug,
    ImplDisplayForError,
    ImplGetSourceFromTufaCommon,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
)]
pub enum NetCheckAvailabilityOriginErrorEnum {
    ReqwestGetOrigin(Error),
    ClientOrigin(StatusCode),
    ServerOrigin(StatusCode),
}

pub async fn net_check_availability(
    link: &str,
    should_trace: bool,
) -> Result<(), Box<NetCheckAvailabilityWrapperError>> {
    match reqwest::get(link).await {
        Err(e) => Err(Box::new(
            NetCheckAvailabilityWrapperError::init_error_with_possible_trace(
                NetCheckAvailabilityOriginErrorEnum::ReqwestGetOrigin(e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                },
                &CONFIG.source_place_type,
                should_trace,
            ),
        )),
        Ok(res) => {
            let status = res.status();
            if status.is_client_error() {
                return Err(Box::new(
                    NetCheckAvailabilityWrapperError::init_error_with_possible_trace(
                        NetCheckAvailabilityOriginErrorEnum::ClientOrigin(status),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            file: String::from(file!()),
                            line: line!(),
                            column: column!(),
                            git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                        },
                        &CONFIG.source_place_type,
                        should_trace,
                    ),
                ));
            }
            if status.is_server_error() {
                return Err(Box::new(
                    NetCheckAvailabilityWrapperError::init_error_with_possible_trace(
                        NetCheckAvailabilityOriginErrorEnum::ServerOrigin(status),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            file: String::from(file!()),
                            line: line!(),
                            column: column!(),
                            git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                        },
                        &CONFIG.source_place_type,
                        should_trace,
                    ),
                ));
            }
            Ok(())
        }
    }
}
