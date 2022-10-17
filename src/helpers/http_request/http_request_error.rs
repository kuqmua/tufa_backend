use crate::helpers::http_request::client_builder_error::ClientBuilderError;
use crate::helpers::http_request::get::bytes::http_request_bytes_error::HttpRequestBytesError;
use crate::helpers::http_request::get::content_length::http_request_content_length_error::HttpRequestContentLengthError;
use crate::helpers::http_request::get::json::http_request_json_error::HttpRequestJsonError;
use crate::helpers::http_request::get::remote_addr::http_request_remote_addr_error::HttpRequestRemoteAddrError;
use crate::helpers::http_request::get::text::http_request_text_error::HttpRequestTextError;
use crate::helpers::http_request::get::text_with_charset::http_request_text_with_charset_error::HttpRequestTextWithCharsetError;
use crate::helpers::http_request::get::version::http_request_version_error::HttpRequestVersionError;
use crate::lazy_static::config::CONFIG;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWas;
use impl_get_source_with_method::ImplGetSourceWithMethod;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitError,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWas,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
)]
pub struct HttpRequestError {
    source: HttpRequestErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithMethod, ImplDisplayForSimpleErrorEnum)]
pub enum HttpRequestErrorEnum {
    ClientBuilder(ClientBuilderError),
    RequestBuilder(HttpRequestBuilderErrorEnum),
}

#[derive(Debug, ImplGetSourceWithMethod, ImplDisplayForSimpleErrorEnum)]
pub enum HttpRequestBuilderErrorEnum {
    // Delete(),
    Get(HttpRequestGetErrorEnum),
    // Head(),
    // Patch(),
    // Post(),
    // Put(),
}

#[derive(Debug, ImplGetSourceWithMethod, ImplDisplayForSimpleErrorEnum)]
pub enum HttpRequestGetErrorEnum {
    Bytes(HttpRequestBytesError),
    ContentLength(HttpRequestContentLengthError),
    Json(HttpRequestJsonError),
    RemoteAddr(HttpRequestRemoteAddrError),
    Text(HttpRequestTextError),
    TextWithCharset(HttpRequestTextWithCharsetError),
    Version(HttpRequestVersionError),
}
