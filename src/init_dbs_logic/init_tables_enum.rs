use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::init_dbs_logic::init_dbs_with_providers_link_parts::init_dbs_with_providers_link_parts;
use crate::init_dbs_logic::init_dbs_with_providers_link_parts::InitDbsProvidersLinkPartsError;
use crate::traits::get_source::GetSource;
use crate::traits::with_tracing::WithTracing;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_get_where_was_for_enum::ImplGetWhereWasForEnum;
// use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum InitTablesEnum {
    ProvidersLinkParts,
}

#[derive(Debug, InitError)] //, ImplGetWhereWasForErrorStruct
pub struct InitTablesError {
    source: InitTablesErrorEnum,
    where_was: WhereWas,
}

impl crate::traits::get_where_was::GetWhereWas for InitTablesError {
    fn get_where_was(&self) -> String {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?} {:#?}", self.where_was, self.source.get_where_was()),
            false => format!("{} {}", self.where_was, self.source.get_where_was()),
        }
    }
}

#[derive(Debug, ImplGetWhereWasForEnum)]
pub enum InitTablesErrorEnum {
    ProvidersLinkParts(InitDbsProvidersLinkPartsError),
}

impl crate::traits::get_source::GetSource for InitTablesErrorEnum {
    fn get_source(&self) -> String {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self),
            false => {
                let mut formatted = match self {
                    InitTablesErrorEnum::ProvidersLinkParts(e) => e.get_source(),
                };
                if !formatted.is_empty() {
                    formatted.pop();
                }
                formatted
            }
        }
    }
}

impl crate::traits::with_tracing::WithTracing<InitTablesErrorEnum> for InitTablesError {
    fn with_tracing(source: InitTablesErrorEnum, where_was: WhereWas) -> Self {
        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = source.get_source(),
                    source_place = where_was.file_line_column(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = source.get_source(),
                    github_source_place = where_was.github_source_place(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = source.get_source());
            }
        }
        Self { source, where_was }
    }
}

impl crate::traits::get_source::GetSource for InitTablesError {
    fn get_source(&self) -> String {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self.source),
            false => self.source.get_source(),
        }
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
                    let where_was = WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    };
                    match should_trace {
                        true => {
                            return Err(Box::new(InitTablesError::with_tracing(
                                InitTablesErrorEnum::ProvidersLinkParts(*e),
                                where_was,
                            )));
                        }
                        false => {
                            return Err(Box::new(InitTablesError::new(
                                InitTablesErrorEnum::ProvidersLinkParts(*e),
                                where_was,
                            )));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
