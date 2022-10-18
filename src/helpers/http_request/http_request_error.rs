use crate::helpers::http_request::async_client_builder::async_client_builder;
use crate::helpers::http_request::client_builder_error::ClientBuilderError;
use crate::helpers::http_request::get::bytes::http_request_bytes_error::HttpRequestBytesError;
use crate::helpers::http_request::get::content_length::http_request_content_length_error::HttpRequestContentLengthError;
use crate::helpers::http_request::get::json::http_request_json_error::HttpRequestJsonError;
use crate::helpers::http_request::get::remote_addr::http_request_remote_addr_error::HttpRequestRemoteAddrError;
use crate::helpers::http_request::get::text::http_request_text_error::HttpRequestTextError;
use crate::helpers::http_request::get::text_with_charset::http_request_text_with_charset_error::HttpRequestTextWithCharsetError;
use crate::helpers::http_request::get::version::http_request_version_error::HttpRequestVersionError;

use crate::helpers::http_request::get::bytes::async_http_request_bytes::async_http_request_bytes;
use crate::helpers::http_request::get::content_length::async_http_request_content_length::async_http_request_content_length;
use crate::helpers::http_request::get::json::async_http_request_json::async_http_request_json;
use crate::helpers::http_request::get::remote_addr::async_http_request_remote_addr::async_http_request_remote_addr;
use crate::helpers::http_request::get::text::async_http_request_text::async_http_request_text;
use crate::helpers::http_request::get::text_with_charset::async_http_request_text_with_charset::async_http_request_text_with_charset;
use crate::helpers::http_request::get::version::async_http_request_version::async_http_request_version;

use crate::helpers::http_request::get::bytes::sync_http_request_bytes::sync_http_request_bytes;
use crate::helpers::http_request::get::content_length::sync_http_request_content_length::sync_http_request_content_length;
use crate::helpers::http_request::get::json::sync_http_request_json::sync_http_request_json;
use crate::helpers::http_request::get::remote_addr::sync_http_request_remote_addr::sync_http_request_remote_addr;
use crate::helpers::http_request::get::text::sync_http_request_text::sync_http_request_text;
use crate::helpers::http_request::get::text_with_charset::sync_http_request_text_with_charset::sync_http_request_text_with_charset;
use crate::helpers::http_request::get::version::sync_http_request_version::sync_http_request_version;

use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::{DateTime, FixedOffset, Local, Utc};
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas;
use impl_get_source_with_method::ImplGetSourceWithMethod;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethod;
use init_error::InitError;
use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

use super::sync_client_builder::sync_client_builder;

#[derive(
    Debug,
    InitError,
    ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
)]
pub struct HttpRequestError {
    source: HttpRequestErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug,
    ImplGetSourceWithMethod,
    ImplDisplayForSimpleErrorEnum,
    ImplGetWhereWasOneOrManyWithMethod,
)]
pub enum HttpRequestErrorEnum {
    ClientBuilder(ClientBuilderError),
    RequestBuilder(HttpRequestBuilderErrorEnum),
}

#[derive(
    Debug,
    ImplGetSourceWithMethod,
    ImplDisplayForSimpleErrorEnum,
    ImplGetWhereWasOneOrManyWithMethod,
)]
pub enum HttpRequestBuilderErrorEnum {
    // Delete(),
    Get(HttpRequestGetErrorEnum),
    // Head(),
    // Patch(),
    // Post(),
    // Put(),
}

#[derive(
    Debug,
    ImplGetSourceWithMethod,
    ImplDisplayForSimpleErrorEnum,
    ImplGetWhereWasOneOrManyWithMethod,
)]
pub enum HttpRequestGetErrorEnum {
    Bytes(HttpRequestBytesError),
    ContentLength(HttpRequestContentLengthError),
    Json(HttpRequestJsonError),
    RemoteAddr(HttpRequestRemoteAddrError),
    Text(HttpRequestTextError),
    TextWithCharset(HttpRequestTextWithCharsetError),
    Version(HttpRequestVersionError),
}

pub async fn async_http_request(
    should_trace: bool,
) -> Result<reqwest::Client, Box<HttpRequestError>> {
    match async_client_builder(reqwest::Client::builder(), false).await {
        Err(e) => Err(Box::new(HttpRequestError::init_error_with_possible_trace(
            HttpRequestErrorEnum::ClientBuilder(*e),
            WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
            &CONFIG.source_place_type,
            &GIT_INFO.data,
            should_trace,
        ))),
        Ok(request_builder) => Ok(request_builder),
    }
}

pub async fn sync_http_request(
    should_trace: bool,
) -> Result<reqwest::blocking::Client, Box<HttpRequestError>> {
    match sync_client_builder(reqwest::blocking::Client::builder(), false) {
        Err(e) => Err(Box::new(HttpRequestError::init_error_with_possible_trace(
            HttpRequestErrorEnum::ClientBuilder(*e),
            WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
            &CONFIG.source_place_type,
            &GIT_INFO.data,
            should_trace,
        ))),
        Ok(request_builder) => Ok(request_builder),
    }
}

pub async fn async_http_request_handle(
    should_trace: bool,
) -> Result<reqwest::Client, Box<HttpRequestError>> {
    match async_client_builder(
        //https://docs.rs/reqwest/0.11.12/reqwest/struct.ClientBuilder.html
        reqwest::Client::builder().no_gzip(),
        false,
    )
    .await
    {
        Err(e) => Err(Box::new(HttpRequestError::init_error_with_possible_trace(
            HttpRequestErrorEnum::ClientBuilder(*e),
            WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
            &CONFIG.source_place_type,
            &GIT_INFO.data,
            should_trace,
        ))),
        Ok(request_builder) => {
            match async_http_request_text(
                // https://docs.rs/reqwest/0.11.12/reqwest/struct.RequestBuilder.html
                request_builder.get("kekw"),
                false,
            )
            .await
            {
                Err(e) => Err(Box::new(HttpRequestError::init_error_with_possible_trace(
                    HttpRequestErrorEnum::RequestBuilder(HttpRequestBuilderErrorEnum::Get(
                        HttpRequestGetErrorEnum::Text(*e),
                    )),
                    WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                    &CONFIG.source_place_type,
                    &GIT_INFO.data,
                    should_trace,
                ))),
                Ok(_) => todo!(),
            }
        }
    }
}
