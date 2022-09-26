use crate::init_dbs_logic::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;
use crate::init_dbs_logic::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsError;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_get_source_for_source_error_enum::ImplGetSourceForSourceErrorEnum;
use impl_get_source_for_struct_with_method::ImplGetSourceForStructWithMethod;
use impl_get_where_was_one_or_many_for_enum::ImplGetWhereWasOneOrManyForEnum;
use impl_get_where_was_one_or_many_for_struct_with_source_enum_method::ImplGetWhereWasOneOrManyForStructWithSourceEnumMethod;
use init_error::InitError;
use init_error_with_tracing::InitErrorWithTracing;
use strum_macros::EnumIter;
use tufa_common::traits::get_bunyan_with_additional_where_was::GetBunyanWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[derive(Debug, EnumIter)]
pub enum InitTablesEnum {
    ProvidersLinkParts,
}

#[derive(
    Debug,
    InitError,
    InitErrorWithTracing,
    ImplGetSourceForStructWithMethod,
    ImplGetWhereWasOneOrManyForStructWithSourceEnumMethod,
)]
pub struct InitTablesError {
    source: InitTablesErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetWhereWasOneOrManyForEnum, ImplGetSourceForSourceErrorEnum)]
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
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
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
