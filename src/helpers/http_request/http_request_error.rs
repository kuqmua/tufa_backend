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

pub async fn async_http_request_wrapper<
    UserAgentValueGeneric,
    CookieProviderGeneric: reqwest::cookie::CookieStore + 'static,
    PoolIdleTimeoutDurationGeneric,
    LocalAddressGeneric,
    TcpKeepaliveGeneric,
>(
    url: &str,
    // headers_argument Option<HeaderMap<HeaderValue>>,
    // user_agent_argument Option<UserAgentValueGeneric>,
    // default_headers_argument Option<HeaderMap>,
    // cookie_store_argument Option<bool>,
    // cookie_provider_argument Option<CookieProviderGeneric>,
    // gzip_argument Option<bool>,
    // brotli_argument Option<bool>,
    // deflate_argument Option<bool>,
    // no_gzip_argument Option<()>,
    // no_brotli_argument Option<()>,
    // no_deflate_argument Option<()>,
    // redirect_argument Option<Policy>,
    // referer_argument Option<bool>,
    // proxy_argument Option<Proxy>,
    // no_proxy_argument Option<()>,
    // timeout_argument Option<Duration>,
    // connect_timeout_argument Option<Duration>,
    // connection_verbose_argument Option<bool>,
    // pool_idle_timeout_argument Option<PoolIdleTimeoutDurationGeneric>,
    // pool_max_idle_per_host_argument Option<usize>,
    // http1_title_case_headers_argument Option<()>,
    // http1_allow_obsolete_multiline_headers_in_responses_argument Option<bool>,
    // http1_only_argument Option<()>,
    // http09_responses_argument Option<()>,
    // http2_prior_knowledge_argument Option<()>,
    // http2_initial_stream_window_size_argument Option<impl Into<Option<u32>>>,
    // http2_initial_connection_window_size_argument Option<impl Into<Option<u32>>>,
    // http2_adaptive_window_argument Option<bool>,
    // http2_max_frame_size_argument Option<impl Into<Option<u32>>>,
    // http2_keep_alive_interval_argument Option<impl Into<Option<Duration>>>,
    // http2_keep_alive_timeout_argument Option<Duration>,
    // http2_keep_alive_while_idle_argument Option<bool>,
    // tcp_nodelay_argument Option<bool>,
    // local_address_argument Option<LocalAddressGeneric>,
    // tcp_keepalive_argument Option<TcpKeepaliveGeneric>,
    // add_root_certificate_argument Option<Certificate>,
    // tls_built_in_root_certs_argument Option<bool>,
    // identity_argument Option<Identity>,
    // danger_accept_invalid_hostnames_argument Option<bool>,
    // danger_accept_invalid_certs_argument Option<bool>,
    // min_tls_version_argument Option<Version>,
    // max_tls_version_argument Option<Version>,
    // use_native_tls_argument Option<()>,
    // use_rustls_tls_argument Option<()>,
    // use_preconfigured_tls_argument Option<impl Any>,
    // trust_dns_argument Option<bool>,
    // no_trust_dns_argument Option<()>,
    // https_only_argument Option<()>,
    // resolve_argument Option<(&str, SocketAddr)>,
    // resolve_to_addrs_argument Option<(&str, &[SocketAddr])>,
    should_trace: bool,
) -> Result<String, Box<HttpRequestError>>
// where
//     UserAgentValueGeneric: TryInto<HeaderValue>,
//     UserAgentValueGeneric::Error: Into<actix_web::http::Error>,
//     PoolIdleTimeoutDurationGeneric: Into<Option<Duration>>,
//     LocalAddressGeneric: Into<Option<IpAddr>>,
//     TcpKeepaliveGeneric: Into<Option<Duration>>,
{
    match async_client_builder(
        //https://docs.rs/reqwest/0.11.12/reqwest/struct.ClientBuilder.html
        reqwest::Client::builder(),
        // user_agent:
        // default_headers:
        // cookie_store:
        // cookie_provider:
        // gzip:
        // brotli:
        // deflate:
        // no_gzip:
        // no_brotli:
        // no_deflate:
        // redirect:
        // referer:
        // proxy:
        // no_proxy:
        // timeout:
        // connect_timeout:
        // connection_verbose:
        // pool_idle_timeout:
        // pool_max_idle_per_host:
        // http1_title_case_headers:
        // http1_allow_obsolete_multiline_headers_in_responses:
        // http1_only:
        // http09_responses:
        // http2_prior_knowledge:
        // http2_initial_stream_window_size:
        // http2_initial_connection_window_size:
        // http2_adaptive_window:
        // http2_max_frame_size:
        // http2_keep_alive_interval:
        // http2_keep_alive_timeout:
        // http2_keep_alive_while_idle:
        // tcp_nodelay:
        // local_address:
        // tcp_keepalive:
        // add_root_certificate:
        // tls_built_in_root_certs:
        // identity:
        // danger_accept_invalid_hostnames:
        // danger_accept_invalid_certs:
        // min_tls_version:
        // max_tls_version:
        // use_native_tls:
        // use_rustls_tls:
        // use_preconfigured_tls:
        // trust_dns:
        // no_trust_dns:
        // https_only:
        // resolve:
        // resolve_to_addrs:
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
                request_builder.get(url),
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
                Ok(string_content) => Ok(string_content),
            }
        }
    }
}
