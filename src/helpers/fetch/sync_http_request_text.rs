use crate::helpers::fetch::http_request_text_error::HttpRequestTextErrorEnum;
use crate::lazy_static::config::CONFIG;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use tufa_common::where_was::WhereWas;
// //
// use crate::init_dbs_logic::init_dbs::init_dbs;
// use crate::init_dbs_logic::init_dbs::InitDbsError;
// use crate::lazy_static::git_info::GIT_INFO;
// use crate::preparation::check_availability::check_availability;
// use crate::preparation::check_availability::CheckAvailabilityError;
// use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas;
// use impl_get_source_with_method::ImplGetSourceWithMethod;
// use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethod;
// use init_error::InitError;
// use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
// use tufa_common::traits::get_source::GetSource;
// use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;

// #[derive(
//     Debug,
//     InitError,
//     ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas,
//     ImplGetWhereWasOneOrManyWithMethod,
// )]
// pub struct PreparationError {
//     source: PreparationErrorEnum,
//     where_was: WhereWas,
// }

// #[derive(Debug, ImplGetWhereWasOneOrManyWithMethod, ImplGetSourceWithMethod)]
// pub enum PreparationErrorEnum {
//     CheckAvailability(CheckAvailabilityError),
//     InitDbs(InitDbsError),
// }
// //

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn sync_http_request_text(link: &str) -> Result<String, Box<HttpRequestTextErrorEnum>> {
    match reqwest::blocking::get(link) {
        Err(e) => Err(Box::new(HttpRequestTextErrorEnum::ReqwestBlockingGet {
            source: e,
            where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        })),
        Ok(res) => {
            if let Err(e) = res.error_for_status_ref() {
                return Err(Box::new(HttpRequestTextErrorEnum::StatusCode {
                    source: e,
                    where_was: WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                }));
            }
            match res.text() {
                Err(e) => Err(Box::new(HttpRequestTextErrorEnum::ResponseText {
                    source: e,
                    where_was: WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                })),
                Ok(text) => Ok(text),
            }
        }
    }
}
