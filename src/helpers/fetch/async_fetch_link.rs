use crate::helpers::fetch::fetch_link_error::FetchLinkErrorEnum;
use crate::lazy_static::config::CONFIG;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use tufa_common::where_was::WhereWas;

//
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
//
#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn async_fetch_link(link: &str) -> Result<String, Box<FetchLinkErrorEnum>> {
    match reqwest::get(link).await {
        Err(e) => Err(Box::new(FetchLinkErrorEnum::ReqwestBlockingGet {
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
                return Err(Box::new(FetchLinkErrorEnum::StatusCode {
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
            match res.text().await {
                Err(e) => Err(Box::new(FetchLinkErrorEnum::ResponseText {
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
