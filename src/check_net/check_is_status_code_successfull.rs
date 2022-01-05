use reqwest::StatusCode;
use std::fmt;

#[derive(thiserror::Error, displaydoc::Display, Debug, BoxErrFromErrDerive)]
pub struct StatusCodeError(
    /// check status code error `{0}`
    Box<StatusCode>,
);

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_is_status_code_successfull(status_code: StatusCode) -> Result<(), StatusCodeError> {
    if !StatusCode::is_success(&status_code) {
        return Err(StatusCodeError(Box::new(status_code)));
    }
    Ok(())
}
