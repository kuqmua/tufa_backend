use crate::init_dbs_logic::init_dbs::init_dbs;
use crate::init_dbs_logic::init_dbs::InitDbsError;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::preparation::check_availability::check_availability;
use crate::preparation::check_availability::CheckAvailabilityError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_get_where_was_for_enum::ImplGetWhereWasForEnum;
use init_error::InitError;
use init_error_with_tracing::InitErrorWithTracing;
use std::fmt::Display;
use tufa_common::traits::get_bunyan_where_was::GetBunyanWhereWas;
use tufa_common::traits::get_bunyan_with_additional_where_was::GetBunyanWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
// use tufa_common::traits::with_tracing::WithTracing;
use tufa_common::where_was::WhereWas;
use tufa_common::where_was::WhereWasWithAddition;
// use impl_get_where_was_for_error_struct::ImplGetWhereWasForErrorStruct;

#[derive(Debug, InitError, InitErrorWithTracing)] //ImplGetWhereWasForErrorStruct,
pub struct PreparationError {
    source: PreparationErrorEnum,
    where_was: WhereWas,
}

// impl crate::traits::get_where_was::GetWhereWas for PreparationError {
//     fn get_where_was(&self) -> String {
//         format!("{} {}", self.where_was, self.source.get_where_was())
//     }
// }

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for PreparationError {
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        let mut vec: Vec<WhereWasWithAddition> = self
            .source
            .get_where_was_one_or_many()
            .into_vec()
            .into_iter()
            .fold(vec![], |mut acc, elem| {
                acc.push(elem);
                acc
            });
        vec.push(tufa_common::where_was::WhereWasWithAddition {
            additional_info: None,
            where_was: self.where_was.clone(),
        });
        tufa_common::where_was::WhereWasOneOrMany::Many(vec)
    }
}

// impl PreparationError {
//     pub fn where_was_error_with_tracing_as_string(
//         source: PreparationErrorEnum,
//         where_was: WhereWas,
//     ) -> String {
//         format!(
//             "{} {}",
//             where_was.file_line_column(),
//             source.get_bunyan_where_was(&CONFIG.source_place_type, &GIT_INFO.data)
//         )
//     }
// }

// impl tufa_common::traits::with_tracing::WithTracing<PreparationErrorEnum> for PreparationError {
//     fn with_tracing(
//         source: PreparationErrorEnum,
//         where_was: WhereWas,
//         source_place_type: &tufa_common::config::source_place_type::SourcePlaceType,
//         git_info: &tufa_common::helpers::git::git_info::GitInformation,
//     ) -> Self {
//         match source_place_type {
//             tufa_common::config::source_place_type::SourcePlaceType::Source => {
//                 tracing::error!(
//                     error = source.get_source(),
//                     where_was = source.get_bunyan_with_additional_where_was(
//                         &where_was,
//                         source_place_type,
//                         git_info,
//                     )
//                 );
//             }
//             tufa_common::config::source_place_type::SourcePlaceType::Github => {
//                 tracing::error!(
//                     error = source.get_source(),
//                     where_was = source.get_bunyan_with_additional_where_was(
//                         &where_was,
//                         source_place_type,
//                         git_info,
//                     )
//                 );
//             }
//             tufa_common::config::source_place_type::SourcePlaceType::None => {
//                 tracing::error!(error = source.get_source());
//             }
//         }
//         Self { source, where_was }
//     }
// }

#[derive(Debug)] //, ImplGetWhereWasForEnum
pub enum PreparationErrorEnum {
    CheckAvailability(CheckAvailabilityError),
    InitDbs(InitDbsError),
}

impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for PreparationErrorEnum {
    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
        match self {
            PreparationErrorEnum::CheckAvailability(e) => e.get_where_was_one_or_many(),
            PreparationErrorEnum::InitDbs(e) => e.get_where_was_one_or_many(),
        }
    }
}

impl PreparationErrorEnum {
    fn get_source(&self) -> String {
        match self {
            PreparationErrorEnum::CheckAvailability(e) => e.get_source(),
            PreparationErrorEnum::InitDbs(e) => e.get_source(),
        }
    }
}

// impl Display for PreparationError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match CONFIG.is_debug_implementation_enable {
//             true => write!(f, "{:#?}", self),
//             false => write!(f, "{:?} {}", self.where_was, self.source),
//         }
//     }
// }

// impl Display for PreparationErrorEnum {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match CONFIG.is_debug_implementation_enable {
//             true => write!(f, "{:#?}", self),
//             false => match self {
//                 PreparationErrorEnum::CheckAvailability(source) => {
//                     write!(f, "{}", *source)
//                 }
//                 PreparationErrorEnum::InitDbs(source) => {
//                     write!(f, "{:#?}", *source)
//                 }
//             },
//         }
//     }
// }

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn prepare_server(should_trace: bool) -> Result<(), Box<PreparationError>> {
    if let Err(e) = check_availability(false).await {
        return Err(Box::new(PreparationError::init_error_with_possible_trace(
            PreparationErrorEnum::CheckAvailability(*e),
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
    //todo: add params dependency function to config after new to check. like if is_mongo_initialization_enabled is true but is_dbs_initialization_enabled is false so is_mongo_initialization_enabled is also false
    if !CONFIG.is_dbs_initialization_enabled
        || (!CONFIG.is_mongo_initialization_enabled && !CONFIG.is_postgres_initialization_enabled)
    {
        return Ok(());
    }
    if let Err(e) = init_dbs(false).await {
        return Err(Box::new(PreparationError::init_error_with_possible_trace(
            PreparationErrorEnum::InitDbs(*e),
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
    Ok(())
}
