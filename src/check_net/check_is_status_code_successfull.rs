use reqwest::StatusCode;
use std::fmt;

#[derive(thiserror::Error, displaydoc::Display, Debug, ImplDisplayDerive)]
pub struct StatusCodeError {
    /// check status code error `{0}`
    source: Box<StatusCodeWrapper>,
}

#[derive(thiserror::Error, displaydoc::Display, Debug, ImplDisplayDerive)]
pub struct StatusCodeWrapper(StatusCode);

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_is_status_code_successfull(status_code: StatusCode) -> Result<(), StatusCodeError> {
    if !StatusCode::is_success(&status_code) {
        return Err(StatusCodeError {
            source: Box::new(StatusCodeWrapper(status_code)),
        });
    }
    Ok(())
}
