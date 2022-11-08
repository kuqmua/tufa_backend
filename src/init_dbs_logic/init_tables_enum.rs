use crate::init_dbs_logic::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;
use crate::init_dbs_logic::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsError;
use crate::once_cell_globals::config::CONFIG;
use crate::once_cell_globals::git_info::GIT_INFO;
use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon;
use impl_get_source_with_method::ImplGetSourceWithMethodFromTufaCommon;
use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethodFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use strum_macros::EnumIter;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;

#[derive(Debug, EnumIter)]
pub enum InitTablesEnum {
    ProvidersLinkParts,
}

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplErrorWithTracingForStructWithGetSourceWithGetWhereWasFromTufaCommon,
    ImplGetSourceWithMethodFromTufaCommon,
    ImplGetWhereWasOneOrManyWithMethodFromTufaCommon,
)]
pub struct InitTablesError {
    source: InitTablesErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug, ImplGetWhereWasOneOrManyWithMethodFromTufaCommon, ImplGetSourceWithMethodFromTufaCommon,
)]
pub enum InitTablesErrorEnum {
    ProvidersLinkParts(InitDbsProvidersLinkPartsError),
}

impl InitTablesEnum {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub async fn init(&self, should_trace: bool) -> Result<(), Box<InitTablesError>> {
        match self {
            InitTablesEnum::ProvidersLinkParts => {
                if let Err(e) = init_dbs_with_providers_link_parts(false).await {
                    return Err(Box::new(InitTablesError::init_error_with_possible_trace(
                        InitTablesErrorEnum::ProvidersLinkParts(*e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    )));
                }
            }
        }
        Ok(())
    }
}
