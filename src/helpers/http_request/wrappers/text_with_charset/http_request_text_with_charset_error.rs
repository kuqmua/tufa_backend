use crate::helpers::http_request::http_request_error::HttpRequestClientRequestBuilderPrepError;
use crate::helpers::http_request::request_builder_methods::text_with_charset::http_request_text_with_charset_error::HttpRequestTextWithCharsetError;
use crate::lazy_static::config::CONFIG;
use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas;
use impl_get_source_with_method::ImplGetSourceWithMethod;
use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethod;
use init_error::InitError;
use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitError,
    ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas,
    ImplGetWhereWasOneOrManyWithMethod,
    ImplGetSourceWithMethod,
)]
pub struct HttpRequestWrapperTextWithCharsetError {
    source: HttpRequestWrapperTextWithCharsetErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithMethod, ImplGetWhereWasOneOrManyWithMethod)]
pub enum HttpRequestWrapperTextWithCharsetErrorEnum {
    Prep(HttpRequestClientRequestBuilderPrepError),
    TextWithCharset(HttpRequestTextWithCharsetError),
}
