use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use git_info::GitInfoFromTufaCommon;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon;
use impl_get_source_with_method::ImplGetSourceWithMethodFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use reqwest::Error;
use reqwest::StatusCode;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;

#[derive(
    Debug,
    ImplDisplayForErrorStruct,
    ImplGetSourceWithMethodFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon,
)]
pub struct NetCheckAvailabilityWrapperError {
    source: NetCheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug,
    GitInfoFromTufaCommon,
    ImplDisplayForSimpleErrorEnum,
    ImplGetSourceWithMethodFromTufaCommon,
)]
pub enum NetCheckAvailabilityErrorEnum {
    ReqwestGetOrigin(Error),
    ClientOrigin(StatusCode),
    ServerOrigin(StatusCode),
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
) -> Result<(), Box<NetCheckAvailabilityWrapperError>> {
    match reqwest::get(link).await {
        Err(e) => Err(Box::new(
            NetCheckAvailabilityWrapperError::init_error_with_possible_trace(
                NetCheckAvailabilityErrorEnum::ReqwestGetOrigin(e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO,
                should_trace,
            ),
        )),
        Ok(res) => {
            let status = res.status();
            if status.is_client_error() {
                return Err(Box::new(
                    NetCheckAvailabilityWrapperError::init_error_with_possible_trace(
                        NetCheckAvailabilityErrorEnum::ClientOrigin(status),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO,
                        should_trace,
                    ),
                ));
            }
            if status.is_server_error() {
                return Err(Box::new(
                    NetCheckAvailabilityWrapperError::init_error_with_possible_trace(
                        NetCheckAvailabilityErrorEnum::ServerOrigin(status),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO,
                        should_trace,
                    ),
                ));
            }
            Ok(())
        }
    }
}
