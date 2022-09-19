use crate::init_dbs_logic::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;
use crate::init_dbs_logic::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsError;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_get_where_was_for_enum::ImplGetWhereWasForEnum;
use tufa_common::traits::get_bunyan_with_additional_where_was::GetBunyanWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::with_tracing::WithTracing;
use tufa_common::where_was::WhereWas;
// use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
use strum_macros::EnumIter;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;

#[derive(Debug, EnumIter)]
pub enum InitTablesEnum {
    ProvidersLinkParts,
}

#[derive(Debug, InitError)] //, ImplGetWhereWasForErrorStruct
pub struct InitTablesError {
    source: InitTablesErrorEnum,
    where_was: WhereWas,
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for InitTablesError {
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        let mut vec = Vec::new();
        self.source
            .get_where_was_one_or_many()
            .into_vec()
            .into_iter()
            .for_each(|w| {
                vec.push(w);
            });
        vec.push(tufa_common::where_was::WhereWasWithAddition {
            additional_info: None,
            where_was: self.where_was.clone(),
        });
        tufa_common::where_was::WhereWasOneOrMany::Many(vec)
    }
}

// impl crate::traits::get_where_was_one_or_many::GetWhereWas for InitTablesError {
//     fn get_where_was(&self) -> String {
//         format!("{} {}", self.where_was, self.source.get_where_was())
//     }
// }

#[derive(Debug)] //, ImplGetWhereWasForEnum
pub enum InitTablesErrorEnum {
    ProvidersLinkParts(InitDbsProvidersLinkPartsError),
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for InitTablesErrorEnum {
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        match self {
            InitTablesErrorEnum::ProvidersLinkParts(e) => e.get_where_was_one_or_many(),
        }
    }
}

impl tufa_common::traits::get_source::GetSource for InitTablesErrorEnum {
    fn get_source(&self) -> String {
        let mut formatted = match self {
            InitTablesErrorEnum::ProvidersLinkParts(e) => e.get_source(),
        };
        if !formatted.is_empty() {
            formatted.pop();
        }
        formatted
    }
}

impl tufa_common::traits::with_tracing::WithTracing<InitTablesErrorEnum> for InitTablesError {
    fn with_tracing(
        source: InitTablesErrorEnum,
        where_was: WhereWas,
        source_place_type: &tufa_common::config::source_place_type::SourcePlaceType,
        git_info: &tufa_common::helpers::git::git_info::GitInformation,
    ) -> Self {
        match source_place_type {
            tufa_common::config::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = source.get_source(),
                    where_was = source.get_bunyan_with_additional_where_was(
                        &where_was,
                        source_place_type,
                        git_info,
                    )
                );
            }
            tufa_common::config::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = source.get_source(),
                    where_was = source.get_bunyan_with_additional_where_was(
                        &where_was,
                        source_place_type,
                        git_info,
                    )
                );
            }
            tufa_common::config::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = source.get_source());
            }
        }
        Self { source, where_was }
    }
}

impl tufa_common::traits::get_source::GetSource for InitTablesError {
    fn get_source(&self) -> String {
        self.source.get_source()
    }
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
