// use crate::helpers::http_request::http_request_error::HttpRequestClientRequestBuilderPrepError;
// use crate::helpers::http_request::request_builder_methods::Extensions::http_request_Extensions_error::ExtensionsError;
// use crate::lazy_static::config::CONFIG;
// use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas;
// use impl_get_source_with_method::ImplGetSourceWithMethod;
// use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethod;
// use init_error::InitError;
// use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
// use tufa_common::traits::get_source::GetSource;
// use tufa_common::traits::where_was_trait::WhereWasTrait;
// use tufa_common::where_was::WhereWas;

// #[derive(
//     Debug,
//     InitError,
//     ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas,
//     ImplGetWhereWasOneOrManyWithMethod,
//     ImplGetSourceWithMethod,
// )]
// pub struct HttpRequestWrapperExtensionsError {
//     source: HttpRequestWrapperExtensionsErrorEnum,
//     where_was: WhereWas,
// }

// #[derive(Debug, ImplGetSourceWithMethod, ImplGetWhereWasOneOrManyWithMethod)]
// pub enum HttpRequestWrapperExtensionsErrorEnum {
//     Prep(HttpRequestClientRequestBuilderPrepError),
//     Extensions(ExtensionsError),
// }
