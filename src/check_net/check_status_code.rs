use reqwest::StatusCode;

use chrono::{DateTime, FixedOffset, Local, Utc};

use crate::helpers::get_git_commit_string::get_git_commit_string;
use crate::traits::git_info_trait::GitInfo;

use crate::helpers::where_was::WhereWas;

#[derive(Debug, GitInfoDerive)]
pub struct CheckStatusCodeError {
    pub source: StatusCode,
    pub where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn check_status_code(status_code: StatusCode) -> Result<(), Box<CheckStatusCodeError>> {
    if !StatusCode::is_success(&status_code) {
        return Err(Box::new(CheckStatusCodeError {
            source: status_code,
            where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(3 * 3600)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        }));
    }
    Ok(())
}
