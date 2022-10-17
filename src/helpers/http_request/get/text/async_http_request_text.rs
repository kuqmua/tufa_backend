use super::http_request_text_error::HttpRequestTextError;
use crate::helpers::http_request::get::text::http_request_text_error::HttpRequestTextErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn async_http_request_text(
    link: &str,
    headers: Option<HeaderMap<HeaderValue>>,
    should_trace: bool,
) -> Result<String, Box<HttpRequestTextError>> {
    let request_builder = match headers {
        Some(h) => reqwest::Client::new().get(link).headers(h),
        None => reqwest::Client::new().get(link),
    };
    match request_builder.send().await {
        Err(e) => Err(Box::new(
            HttpRequestTextError::init_error_with_possible_trace(
                HttpRequestTextErrorEnum::ReqwestGet(e),
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
            ),
        )),
        Ok(res) => {
            if let Err(e) = res.error_for_status_ref() {
                return Err(Box::new(
                    HttpRequestTextError::init_error_with_possible_trace(
                        HttpRequestTextErrorEnum::StatusCode(e),
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
                    ),
                ));
            }
            match res.text().await {
                Err(e) => Err(Box::new(
                    HttpRequestTextError::init_error_with_possible_trace(
                        HttpRequestTextErrorEnum::ResponseText(e),
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
                    ),
                )),
                Ok(text) => Ok(text),
            }
        }
    }
}
