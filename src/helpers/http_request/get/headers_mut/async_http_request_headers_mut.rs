use super::http_request_headers_mut_error::HttpRequestHeadersMutError;
use crate::helpers::http_request::get::headers_mut::http_request_headers_mut_error::HttpRequestHeadersMutErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use reqwest::RequestBuilder;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn async_http_request_headers_mut(
    request_builder: RequestBuilder,
    should_trace: bool,
) -> Result<reqwest::header::HeaderMap, Box<HttpRequestHeadersMutError>> {
    match request_builder.send().await {
        Err(e) => Err(Box::new(
            HttpRequestHeadersMutError::init_error_with_possible_trace(
                HttpRequestHeadersMutErrorEnum::ReqwestGet(e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        )),
        Ok(mut res) => {
            if let Err(e) = res.error_for_status_ref() {
                return Err(Box::new(
                    HttpRequestHeadersMutError::init_error_with_possible_trace(
                        HttpRequestHeadersMutErrorEnum::StatusCode(e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    ),
                ));
            }
            Ok(res.headers_mut().clone()) //todo do something with it
        }
    }
}
