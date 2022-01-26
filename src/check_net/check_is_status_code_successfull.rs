use reqwest::StatusCode;

#[derive(Debug)]
pub struct StatusCodeError {
    source: Box<StatusCode>,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_is_status_code_successfull(status_code: StatusCode) -> Result<(), StatusCodeError> {
    if !StatusCode::is_success(&status_code) {
        return Err(StatusCodeError {
            source: Box::new(status_code),
        });
    }
    Ok(())
}
