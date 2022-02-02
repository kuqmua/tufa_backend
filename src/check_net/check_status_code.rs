use reqwest::StatusCode;

use crate::helpers::get_git_commit_string::get_git_commit_string;
use crate::traits::git_info_trait::GitInfo;

#[derive(Debug, GitInfoDerive)]
pub struct CheckStatusCodeError {
    source: StatusCode,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_status_code(status_code: StatusCode) -> Result<(), Box<CheckStatusCodeError>> {
    if !StatusCode::is_success(&status_code) {
        return Err(Box::new(CheckStatusCodeError {
            source: status_code,
            file: file!(),
            line: line!(),
            column: column!(),
        }));
    }
    Ok(())
}
