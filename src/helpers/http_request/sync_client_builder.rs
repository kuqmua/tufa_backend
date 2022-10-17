use crate::helpers::http_request::client_builder_error::ClientBuilderError;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn sync_client_builder(
    client_builder: reqwest::blocking::ClientBuilder,
    should_trace: bool,
) -> Result<reqwest::blocking::Client, Box<ClientBuilderError>> {
    match client_builder.build() {
        Err(e) => Err(Box::new(
            ClientBuilderError::init_error_with_possible_trace(
                e,
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
        Ok(request_builder) => Ok(request_builder),
    }
}
