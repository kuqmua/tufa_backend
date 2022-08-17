use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::git_info::GIT_INFO;
use std::fmt::Display;
extern crate chrono;
use chrono::prelude::DateTime;
use chrono::FixedOffset;
use tracing::error;

#[derive(Debug, Clone)]
pub struct WhereWas {
    time: DateTime<FixedOffset>,
    file: &'static str,
    line: u32,
    column: u32,
    stack: Vec<TimeFileLineColumn>,
}

#[derive(Debug, Clone)]
pub struct TimeFileLineColumn {
    pub time: DateTime<FixedOffset>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl Display for WhereWas {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match CONFIG.is_debug_implementation_enable {
            true => write!(f, "{:#?}", self),
            false => {
                if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled
                {
                    write!(f, "{}\n{}", self.source_place(), self.github_source_place())
                } else if CONFIG.is_show_source_place_enabled {
                    write!(f, "{}", self.source_place())
                } else if CONFIG.is_show_github_source_place_enabled {
                    write!(f, "{}", self.github_source_place())
                } else {
                    write!(f, "")
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum WhereWasTracing {
    Message(String),
    Child(Vec<WhereWas>),
}

impl WhereWas {
    pub fn new(
        time: DateTime<FixedOffset>,
        file: &'static str,
        line: u32,
        column: u32,
        option_child_or_message: Option<WhereWasTracing>,
    ) -> Self {
        match option_child_or_message {
            Some(child_or_message) => match child_or_message {
                WhereWasTracing::Message(e) => {
                    let self_handle = Self {
                        time,
                        file,
                        line,
                        column,
                        stack: vec![TimeFileLineColumn {
                            time,
                            file,
                            line,
                            column,
                        }],
                    };
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        error!(
                            error = format!("{}", e),
                            source = self_handle.source_place(),
                            github_source = self_handle.github_source_place(),
                        );
                    } else if CONFIG.is_show_source_place_enabled {
                        error!(
                            error = format!("{}", e),
                            source = self_handle.source_place(),
                        );
                    } else if CONFIG.is_show_github_source_place_enabled {
                        error!(
                            error = format!("{}", e),
                            github_source = self_handle.github_source_place(),
                        );
                    } else {
                        error!(error = format!("{}", e),);
                    }
                    self_handle
                }
                WhereWasTracing::Child(where_was_vec) => {
                    let mut children_stack = where_was_vec
                        .iter()
                        .map(|e| TimeFileLineColumn {
                            time: e.time,
                            file: e.file,
                            line: e.line,
                            column: e.column,
                        })
                        .collect::<Vec<TimeFileLineColumn>>();
                    children_stack.push(TimeFileLineColumn {
                        time,
                        file,
                        line,
                        column,
                    });
                    Self {
                        time,
                        file,
                        line,
                        column,
                        stack: children_stack,
                    }
                }
            },
            None => Self {
                time,
                file,
                line,
                column,
                stack: vec![TimeFileLineColumn {
                    time,
                    file,
                    line,
                    column,
                }],
            },
        }
        // let s = Self {
        //     time,
        //     file,
        //     line,
        //     column,
        // };
        // if let Some(child_or_message) = option_child_or_message {
        //     // s.tracing_trace(child_or_message);
        //     // s.tracing_debug(child_or_message);
        //     // s.tracing_info(child_or_message);
        //     // s.tracing_warn(child_or_message);
        //     s.tracing_error(child_or_message);
        // }
        // s
    }
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
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn github_source_place(&self) -> String {
        GIT_INFO.get_git_source_file_link(self.file, self.line)
    }
    fn tracing_error(&self, child_or_error: WhereWasTracing) {
        //impl std::error::Error
        match child_or_error {
            WhereWasTracing::Message(e) => {
                if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled
                {
                    error!(
                        error = format!("{}", e),
                        source = self.source_place(),
                        github_source = self.github_source_place(),
                    );
                } else if CONFIG.is_show_source_place_enabled {
                    error!(error = format!("{}", e), source = self.source_place(),);
                } else if CONFIG.is_show_github_source_place_enabled {
                    error!(
                        error = format!("{}", e),
                        github_source = self.github_source_place(),
                    );
                } else {
                    error!(error = format!("{}", e),);
                }
            }
            WhereWasTracing::Child(c) => {
                if CONFIG.is_parent_tracing_enabled {
                    let child_sources = c.iter().enumerate().fold(
                        String::from(""),
                        |mut acc, (index, where_was)| {
                            let elem = format!(" {} {}", index, where_was.source_place());
                            acc.push_str(&elem);
                            acc
                        },
                    );
                    let github_sources = c.iter().enumerate().fold(
                        String::from(""),
                        |mut acc, (index, where_was)| {
                            let elem = format!(" {} {}", index, where_was.github_source_place());
                            acc.push_str(&elem);
                            acc
                        },
                    );
                    if CONFIG.is_show_source_place_enabled
                        && CONFIG.is_show_github_source_place_enabled
                    {
                        error!(
                            source = self.source_place(),
                            github_source = self.github_source_place(),
                            child_sources = child_sources,
                            child_github_sources = github_sources,
                        );
                    } else if CONFIG.is_show_source_place_enabled {
                        error!(source = self.source_place(), child_sources = child_sources,);
                    } else if CONFIG.is_show_github_source_place_enabled {
                        error!(
                            github_source = self.github_source_place(),
                            child_github_sources = github_sources,
                        );
                    } else {
                        error!(source = String::from("disabled"));
                    }
                }
            }
        }
    }
}
