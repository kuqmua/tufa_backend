use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::init_dbs_logic::init_tables_enum::InitTablesEnum;
use crate::init_dbs_logic::init_tables_enum::InitTablesError;
use crate::traits::get_source::GetSource;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use futures::future::join_all;
// use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;
use sqlx::types::chrono::Local;
use strum::IntoEnumIterator;

#[derive(Debug)] //ImplGetWhereWasForErrorStruct
pub struct InitDbsError {
    source: Vec<InitTablesError>,
    where_was: WhereWas,
}

impl crate::traits::get_where_was::GetWhereWas for InitDbsError {
    fn get_where_was(&self) -> String {
        let mut formatted_vec = self
            .source
            .iter()
            .map(|error| format!("{} ", error.get_where_was()))
            .fold(String::from(""), |mut acc, elem| {
                acc.push_str(&elem);
                acc
            });
        if !formatted_vec.is_empty() {
            formatted_vec.pop();
        }
        let formatted = format!("[{}]", formatted_vec);
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?} {:#?}", self.where_was, formatted),
            false => format!("{} {}", self.where_was, formatted),
        }
    }
}

impl InitDbsError {
    pub fn with_tracing(source: Vec<InitTablesError>, where_was: WhereWas) -> Self {
        let mut errors = source
            .iter()
            .map(|error| format!("{},", error.get_source()))
            .fold(String::from(""), |mut acc, elem| {
                acc.push_str(&elem);
                acc
            });
        if !errors.is_empty() {
            errors.pop();
        }
        match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
            crate::config_mods::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = errors,
                    source_place = format!("{}", where_was.file_line_column())
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = errors,
                    github_source_place = where_was.github_source_place(),
                );
            }
            crate::config_mods::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = errors);
            }
        }
        Self { source, where_was }
    }
}
//todo implement better type support for derive(InitError)
impl InitDbsError {
    pub fn new(source: Vec<InitTablesError>, where_was: WhereWas) -> Self {
        Self { source, where_was }
    }
}

impl crate::traits::get_source::GetSource for InitDbsError {
    fn get_source(&self) -> String {
        match crate::config_mods::lazy_static_config::CONFIG.is_debug_implementation_enable {
            true => format!("{:#?}", self.source),
            false => {
                let mut formatted = self
                    .source
                    .iter()
                    .map(|error| format!("{},", error.get_source()))
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                    });
                if !formatted.is_empty() {
                    formatted.pop();
                }
                formatted
            }
        }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn init_dbs(should_trace: bool) -> Result<(), Box<InitDbsError>> {
    let results =
        join_all(InitTablesEnum::iter().map(|table| async move { table.init(false).await }))
            .await
            .into_iter()
            .filter_map(|result| {
                if let Err(e) = result {
                    return Some(*e);
                }
                None
            })
            .collect::<Vec<InitTablesError>>();
    if !results.is_empty() {
        let where_was = WhereWas {
            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
            file: file!(),
            line: line!(),
            column: column!(),
        };
        match should_trace {
            true => {
                return Err(Box::new(InitDbsError::with_tracing(results, where_was)));
            }
            false => {
                return Err(Box::new(InitDbsError::new(results, where_was)));
            }
        }
    }
    Ok(())
}
