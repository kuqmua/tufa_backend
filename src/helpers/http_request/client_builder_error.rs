use crate::lazy_static::config::CONFIG;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSource;
use impl_get_source_without_method::ImplGetSourceWithoutMethod;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitError,
    ImplErrorWithTracingForStructWithoutGetSource,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
    ImplGetSourceWithoutMethod,
    ImplDisplayForErrorStruct,
)]
pub struct ClientBuilderError {
    source: reqwest::Error,
    where_was: WhereWas,
}
