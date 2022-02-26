use crate::helpers::get_git_source_file_link::get_git_source_file_link;

extern crate chrono;
use chrono::prelude::DateTime;
use chrono::FixedOffset;

#[derive(Debug)]
pub struct WhereWas {
    pub time: DateTime<FixedOffset>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl WhereWas {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn readable_time(&self) -> String {
        self.time.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn source_place(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn source_place_with_readable_time(&self) -> String {
        format!("{} {}", self.source_place(), self.readable_time())
    }
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn github_source_place(&self) -> String {
        get_git_source_file_link(self.file, self.line)
    }
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn github_source_place_with_readable_time(&self) -> String {
        format!("{} {}", self.github_source_place(), self.readable_time())
    }
}
