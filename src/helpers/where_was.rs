use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::git_info::GIT_INFO;
use std::fmt::Display;
extern crate chrono;
use chrono::prelude::DateTime;
use chrono::FixedOffset;

#[derive(Debug, Clone)]
pub struct WhereWas {
    pub time: DateTime<FixedOffset>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl Display for WhereWas {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => match crate::config_mods::lazy_static_config::CONFIG.source_place_type {
                crate::config_mods::source_place_type::SourcePlaceType::Source => {
                    write!(f, "{}", self.file_line_column())
                }
                crate::config_mods::source_place_type::SourcePlaceType::Github => {
                    write!(f, "{}", self.github_file_line_column())
                }
                crate::config_mods::source_place_type::SourcePlaceType::None => {
                    write!(f, "")
                }
            },
        }
    }
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
    pub fn file_line_column(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn github_file_line_column(&self) -> String {
        GIT_INFO.get_git_source_file_link(self.file, self.line)
    }
}

#[derive(Debug, Clone)]
pub enum WhereWasOneOrMany {
    One(WhereWas),
    Many(Vec<WhereWasWithAddition>),
}

#[derive(Debug, Clone)]
pub struct WhereWasWithAddition {
    pub additional_info: Option<String>,
    pub where_was: WhereWas,
}
