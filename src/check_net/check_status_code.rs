use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::git::get_git_commit_string::get_git_commit_string;
use crate::helpers::where_was::WhereWas;
use crate::traits::git_info_trait::GitInfo;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use git_info::GitInfoDerive;
use reqwest::StatusCode;
use std::fmt;

#[derive(Debug, GitInfoDerive)]
pub struct CheckStatusCodeError {
    pub source: StatusCode,
    pub where_was: WhereWas,
}

impl fmt::Display for CheckStatusCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled {
            write!(
                f,
                "{}\n{}\n{}",
                self.where_was.source_place_with_readable_time(),
                self.where_was.github_source_place_with_readable_time(),
                self.source
            )
        } else if CONFIG.is_show_source_place_enabled {
            write!(
                f,
                "{}\n{}",
                self.where_was.source_place_with_readable_time(),
                self.source
            )
        } else if CONFIG.is_show_github_source_place_enabled {
            write!(
                f,
                "{}\n{}",
                self.where_was.github_source_place_with_readable_time(),
                self.source
            )
        } else {
            write!(f, "{}", self.source)
        }
    }
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
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        }));
    }
    Ok(())
}
