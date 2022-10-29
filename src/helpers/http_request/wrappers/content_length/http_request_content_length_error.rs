use crate::helpers::http_request::http_request_error::HttpRequestClientRequestBuilderPrepError;
use crate::helpers::http_request::request_builder_methods::content_length::content_length_error::ContentLengthError;
use crate::lazy_static::config::CONFIG;
use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon;
use impl_get_source_with_method::ImplGetSourceWithMethodFromTufaCommon;
use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethodFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon,
    ImplGetWhereWasOneOrManyWithMethodFromTufaCommon,
    ImplGetSourceWithMethodFromTufaCommon,
)]
pub struct HttpRequestWrapperContentLengthError {
    source: HttpRequestWrapperContentLengthErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug, ImplGetSourceWithMethodFromTufaCommon, ImplGetWhereWasOneOrManyWithMethodFromTufaCommon,
)]
pub enum HttpRequestWrapperContentLengthErrorEnum {
    Prep(HttpRequestClientRequestBuilderPrepError),
    ContentLength(ContentLengthError),
}
