use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::init_dbs_logic::init_dbs::init_dbs;
use crate::init_dbs_logic::init_dbs::InitDbsError;
use crate::preparation::check_availability::check_availability;
use crate::preparation::check_availability::CheckAvailabilityError;
use crate::traits::get_source::GetSource;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_get_where_was_for_enum::ImplGetWhereWasForEnum;
use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use init_error::InitError;
use std::fmt::Display;

#[derive(Debug, ImplGetWhereWasForErrorStruct, InitError)]
pub struct PreparationError {
    source: PreparationErrorEnum,
    where_was: WhereWas,
}

impl PreparationError {
    pub fn with_tracing(source: PreparationErrorEnum, where_was: WhereWas) -> Self {
        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = source.get_source(),
                    source_place = where_was.source_place(),
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

#[derive(Debug, ImplGetWhereWasForEnum)]
pub enum PreparationErrorEnum {
    CheckAvailability(CheckAvailabilityError),
    InitDbs(InitDbsError),
}

impl PreparationErrorEnum {
    fn get_source(&self) -> String {
        match self {
            PreparationErrorEnum::CheckAvailability(e) => e.get_source(),
            PreparationErrorEnum::InitDbs(e) => e.get_source(),
        }
    }
}

impl Display for PreparationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => write!(f, "{:?} {}", self.where_was, self.source),
        }
    }
}

impl Display for PreparationErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => match self {
                PreparationErrorEnum::CheckAvailability(source) => {
                    write!(f, "{}", *source)
                }
                PreparationErrorEnum::InitDbs(source) => {
                    write!(f, "{:#?}", *source)
                }
            },
        }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn prepare_server(should_trace: bool) -> Result<(), Box<PreparationError>> {
    if let Err(e) = check_availability(false).await {
        let where_was = WhereWas {
            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file: file!(),
            line: line!(),
            column: column!(),
        };
        match should_trace {
            true => {
                return Err(Box::new(PreparationError::with_tracing(
                    PreparationErrorEnum::CheckAvailability(*e),
                    where_was,
                )));
            }
            false => {
                return Err(Box::new(PreparationError::new(
                    PreparationErrorEnum::CheckAvailability(*e),
                    where_was,
                )));
            }
        }
    }
    //todo: add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
    if !CONFIG.is_dbs_initialization_enabled
        || (!CONFIG.is_mongo_initialization_enabled && !CONFIG.is_postgres_initialization_enabled)
    {
        return Ok(());
    }
    if let Err(e) = init_dbs(false).await {
        let where_was = WhereWas {
            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file: file!(),
            line: line!(),
            column: column!(),
        };
        match should_trace {
            true => {
                return Err(Box::new(PreparationError::with_tracing(
                    PreparationErrorEnum::InitDbs(*e),
                    where_was,
                )));
            }
            false => {
                return Err(Box::new(PreparationError::new(
                    PreparationErrorEnum::InitDbs(*e),
                    where_was,
                )));
            }
        }
    }
    Ok(())
}
