use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use git_info::GitInfoFromTufaCommon;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon;
use impl_get_source_with_method::ImplGetSourceWithMethodFromTufaCommon;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use reqwest::Error;
use reqwest::StatusCode;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    ImplDisplayForErrorStruct,
    ImplGetSourceWithMethodFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWasFromTufaCommon,
)]
pub struct NetCheckAvailabilityError {
    source: NetCheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug,
    GitInfoFromTufaCommon,
    ImplDisplayForSimpleErrorEnum,
    ImplGetSourceWithoutMethodFromTufaCommon,
)]
pub enum NetCheckAvailabilityErrorEnum {
    ReqwestGet(Error),
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
        Err(e) => Err(Box::new(
            NetCheckAvailabilityError::init_error_with_possible_trace(
                NetCheckAvailabilityErrorEnum::ReqwestGet(e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        )),
        Ok(res) => {
            let status = res.status();
            if status.is_client_error() {
                return Err(Box::new(
                    NetCheckAvailabilityError::init_error_with_possible_trace(
                        NetCheckAvailabilityErrorEnum::Client(status),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    ),
                ));
            }
            if status.is_server_error() {
                return Err(Box::new(
                    NetCheckAvailabilityError::init_error_with_possible_trace(
                        NetCheckAvailabilityErrorEnum::Server(status),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    ),
                ));
            }
            Ok(())
        }
    }
}
