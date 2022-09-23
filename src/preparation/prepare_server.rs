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
use impl_get_source_for_source_error_enum::ImplGetSourceForSourceErrorEnum;
use impl_get_where_was_one_or_many_for_enum::ImplGetWhereWasOneOrManyForEnum;
use impl_get_where_was_one_or_many_for_struct_with_source_enum_method::ImplGetWhereWasOneOrManyForStructWithSourceEnumMethod;
use init_error::InitError;
use init_error_with_tracing::InitErrorWithTracing;
use tufa_common::traits::get_bunyan_with_additional_where_was::GetBunyanWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug, InitError, InitErrorWithTracing, ImplGetWhereWasOneOrManyForStructWithSourceEnumMethod,
)]
pub struct PreparationError {
    source: PreparationErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetWhereWasOneOrManyForEnum, ImplGetSourceForSourceErrorEnum)]
pub enum PreparationErrorEnum {
    CheckAvailability(CheckAvailabilityError),
    InitDbs(InitDbsError),
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
