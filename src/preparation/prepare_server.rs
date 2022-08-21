use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::mongo::get_mongo_url::get_mongo_url;
use crate::helpers::postgres::get_postgres_url::get_postgres_url;
use crate::helpers::where_was::WhereWas;
use crate::helpers::where_was::WhereWasOneOrFew;
use crate::init_dbs_logic::init_dbs::init_dbs;
use crate::init_dbs_logic::init_tables_enum::InitTablesEnumError;
use crate::preparation::check_availability::check_availability;
use crate::preparation::check_availability::CheckAvailabilityError;
use crate::traits::tufa_error::TufaError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use std::fmt::Display;
// use init_error_with_tracing::DeriveInitErrorWithTracing;
//DeriveInitErrorWithTracing
#[derive(Debug)]
pub struct PreparationError {
    source: PreparationErrorEnum,
    where_was: Vec<WhereWasOneOrFew>,
}

impl PreparationError {
    pub fn with_tracing(
        source: PreparationErrorEnum,
        where_was: Vec<crate::helpers::where_was::WhereWasOneOrFew>,
    ) -> Self {
        if where_was.len() == 1 {
            if let Some(first_value) = where_was.get(0) {
                match first_value {
                    crate::helpers::where_was::WhereWasOneOrFew::One(where_was_one) => {
                        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
                            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                                tracing::error!(
                                    error = format!("{}", source.get_source()),
                                    children_source = format!("{}", source.get_where_was()),
                                    source_place = where_was_one.source_place(),
                                );
                            }
                            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                                tracing::error!(
                                    error = format!("{}", source.get_source()),
                                    children_source = format!("{}", source.get_where_was()),
                                    github_source_place = where_was_one.github_source_place(),
                                );
                            }
                            crate::config_mods::source_place_type::SourcePlaceType::None => {
                                tracing::error!(error = format!("{}", source));
                            }
                        }
                    }
                    crate::helpers::where_was::WhereWasOneOrFew::Few(hs_where_was) => {
                        tracing::error!(error = "todo WhereWasOneOrFew::Few",)
                    }
                }
            }
            //todo next elements
        }
        Self { source, where_was }
    }
}

#[derive(Debug)]
pub enum PreparationErrorEnum {
    CheckAvailability(Box<CheckAvailabilityError>),
    InitDbs(Vec<Box<InitTablesEnumError>>),
}

impl TufaError for PreparationErrorEnum {
    fn get_source(&self) -> String {
        match self {
            PreparationErrorEnum::CheckAvailability(e) => e.get_source(),
            PreparationErrorEnum::InitDbs(e) => {
                let mut content = e.iter().fold(String::from(""), |mut acc, elem| {
                    acc.push_str(&format!("{:?},", *elem)); //todo
                    acc
                });
                if !content.is_empty() {
                    content.pop();
                }
                content
            }
        }
    }
    fn get_where_was(&self) -> String {
        match self {
            PreparationErrorEnum::CheckAvailability(e) => e.get_where_was(),
            PreparationErrorEnum::InitDbs(e) => {
                let mut content = e.iter().fold(String::from(""), |mut acc, elem| {
                    acc.push_str(&format!("{:?},", *elem)); //todo .get_where_was()
                    acc
                });
                if !content.is_empty() {
                    content.pop();
                }
                content
            }
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
pub async fn prepare_server() -> Result<(), Box<PreparationError>> {
    if let Err(e) = check_availability(false).await {
        let where_was = WhereWas {
            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file: file!(),
            line: line!(),
            column: column!(),
        };
        return Err(Box::new(PreparationError::with_tracing(
            PreparationErrorEnum::CheckAvailability(e),
            vec![WhereWasOneOrFew::One(where_was)],
        )));
    }
    //todo: add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
    if !CONFIG.is_dbs_initialization_enabled
        || (!CONFIG.is_mongo_initialization_enabled && !CONFIG.is_postgres_initialization_enabled)
    {
        return Ok(());
    }
    if let Err(e) = init_dbs().await {
        let where_was = WhereWas {
            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file: file!(),
            line: line!(),
            column: column!(),
        };
        return Err(Box::new(PreparationError::with_tracing(
            PreparationErrorEnum::InitDbs(e),
            vec![WhereWasOneOrFew::One(where_was)],
        )));
    }
    Ok(())
}
