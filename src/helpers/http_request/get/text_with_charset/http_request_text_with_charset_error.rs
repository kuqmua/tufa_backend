use crate::lazy_static::config::CONFIG;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWas;
use impl_get_source_without_method::ImplGetSourceWithoutMethod;
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
    ImplGetSourceWithoutMethod,
    ImplDisplayForErrorStruct,
)]
pub struct HttpRequestTextWithCharsetError {
    source: HttpRequestTextWithCharsetErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethod, ImplDisplayForSimpleErrorEnum)]
pub enum HttpRequestTextWithCharsetErrorEnum {
    ReqwestGet(reqwest::Error),
    StatusCode(reqwest::Error),
    ResponseTextWithCharset(reqwest::Error),
}
