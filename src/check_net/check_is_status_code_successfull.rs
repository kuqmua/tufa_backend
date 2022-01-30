use reqwest::StatusCode;

use crate::helpers::lazy_static_git_info::GIT_INFO;
use crate::traits::git_info_trait::GitInfo;

#[derive(Debug, GitInfoDerive)]
pub struct StatusCodeError {
    source: StatusCode,
    line: String,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_is_status_code_successfull(
    status_code: StatusCode,
) -> Result<(), Box<StatusCodeError>> {
    if !StatusCode::is_success(&status_code) {
        return Err(Box::new(StatusCodeError {
            source: status_code,
            line: format!("{}:{}:{}", file!(), line!(), column!()),
        }));
    }
    Ok(())
}
