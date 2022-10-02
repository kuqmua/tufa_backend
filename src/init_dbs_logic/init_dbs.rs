use crate::init_dbs_logic::init_tables_enum::InitTablesEnum;
use crate::init_dbs_logic::init_tables_enum::InitTablesError;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use futures::future::join_all;
use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas;
use impl_get_source_for_struct_with_method::ImplGetSourceForStructWithMethod;
use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethod;
use init_error::InitError;
use sqlx::types::chrono::Local;
use strum::IntoEnumIterator;
use tufa_common::traits::get_bunyan_where_was::GetBunyanWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitError,
    ImplGetWhereWasOneOrManyWithMethod,
    ImplGetSourceForStructWithMethod,
    ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas,
)]
pub struct InitDbsError {
    source: Vec<InitTablesError>,
    where_was: WhereWas,
}

// impl tufa_common::traits::with_tracing::WithTracing<Vec<InitTablesError>> for InitDbsError {
//     fn with_tracing(
//         source: Vec<InitTablesError>,
//         where_was: tufa_common::where_was::WhereWas,
//         source_place_type: &tufa_common::config::source_place_type::SourcePlaceType,
//         git_info: &tufa_common::helpers::git::git_info::GitInformation,
//     ) -> Self {
//         match source_place_type {
//             tufa_common::config::source_place_type::SourcePlaceType::Source => {
//                 let mut error_handle = source.iter().map(|e| e.get_source()).fold(
//                     String::from(""),
//                     |mut acc, elem| {
//                         acc.push_str(&elem);
//                         acc
//                     },
//                 );
//                 if !error_handle.is_empty() {
//                     error_handle.pop();
//                     error_handle.pop();
//                 }
//                 let mut where_was_vec_as_string = source
//                     .iter()
//                     .map(|(key, e)| {
//                         format!("{}, ", e.get_bunyan_where_was(source_place_type, git_info))
//                     })
//                     .fold(String::from(""), |mut acc, elem| {
//                         acc.push_str(&elem);
//                         acc
//                     });
//                 if !where_was_vec_as_string.is_empty() {
//                     where_was_vec_as_string.pop();
//                     where_was_vec_as_string.pop();
//                 }
//                 let where_was_handle = format!(
//                     "[{} from [{}]]",
//                     where_was.file_line_column(),
//                     where_was_vec_as_string
//                 );
//                 let where_was_handle = String::from("kekw");
//                 tracing::error!(error = error_handle, where_was = where_was_handle,);
//             }
//             tufa_common::config::source_place_type::SourcePlaceType::Github => {
//                 let mut error_handle = source.iter().map(|e| e.get_source()).fold(
//                     String::from(""),
//                     |mut acc, elem| {
//                         acc.push_str(&elem);
//                         acc
//                     },
//                 );
//                 if !error_handle.is_empty() {
//                     error_handle.pop();
//                     error_handle.pop();
//                 }
//                 let mut where_was_vec_as_string = source
//                     .iter()
//                     .map(|(key, e)| {
//                         format!("{}, ", e.get_bunyan_where_was(source_place_type, git_info))
//                     })
//                     .fold(String::from(""), |mut acc, elem| {
//                         acc.push_str(&elem);
//                         acc
//                     });
//                 if !where_was_vec_as_string.is_empty() {
//                     where_was_vec_as_string.pop();
//                     where_was_vec_as_string.pop();
//                 }
//                 let where_was_handle = format!(
//                     "[{} from [{}]]",
//                     where_was.file_line_column(),
//                     where_was_vec_as_string
//                 );
//                 tracing::error!(error = error_handle, where_was = where_was_handle,);
//             }
//             tufa_common::config::source_place_type::SourcePlaceType::None => {
//                 let mut error_handle = source.iter().map(|e| format!("{}, ", e.get_source())).fold(
//                     String::from(""),
//                     |mut acc, elem| {
//                         acc.push_str(&elem);
//                         acc
//                     },
//                 );
//                 if !error_handle.is_empty() {
//                     error_handle.pop();
//                     error_handle.pop();
//                 }
//                 tracing::error!(error = error_handle);
//             }
//         };
//         Self { source, where_was }
//     }
// }

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
        return Err(Box::new(InitDbsError::init_error_with_possible_trace(
            results,
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
