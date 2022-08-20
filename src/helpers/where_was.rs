use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::git_info::GIT_INFO;
use std::collections::HashMap;
use std::fmt::Display;
extern crate chrono;
use chrono::prelude::DateTime;
use chrono::FixedOffset;

#[derive(Debug, Clone)]
pub enum WhereWasOneOrFew {
    One(WhereWas),
    Few(HashMap<String, WhereWas>),
}

// impl Display for WhereWasOneOrFew {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             WhereWasOneOrFew::One(where_was) => match CONFIG.is_debug_implementation_enable {
//                 true => write!(f, "{:#?}", self),
//                 false => {
//                     if CONFIG.is_show_source_place_enabled
//                         && CONFIG.is_show_github_source_place_enabled
//                     {
//                         write!(
//                             f,
//                             "{}\n{}",
//                             where_was.source_place(),
//                             where_was.github_source_place()
//                         )
//                     } else if CONFIG.is_show_source_place_enabled {
//                         write!(f, "{}", where_was.source_place())
//                     } else if CONFIG.is_show_github_source_place_enabled {
//                         write!(f, "{}", where_was.github_source_place())
//                     } else {
//                         write!(f, "")
//                     }
//                 }
//             },
//             WhereWasOneOrFew::Few(hm_where_was) => match CONFIG.is_debug_implementation_enable {
//                 true => write!(f, "{:#?}", self),
//                 false => {
//                     if CONFIG.is_show_source_place_enabled
//                         && CONFIG.is_show_github_source_place_enabled
//                     {
//                         let content = hm_where_was.clone().iter().fold(
//                             String::from(""),
//                             |mut acc, (key, value)| {
//                                 acc.push_str(format!(
//                                     "{}\n{}",
//                                     where_was.source_place(),
//                                     where_was.github_source_place()
//                                 ));
//                                 acc
//                             },
//                         );
//                         write!(
//                             f,
//                             "{}\n{}",
//                             where_was.source_place(),
//                             where_was.github_source_place()
//                         )
//                     } else if CONFIG.is_show_source_place_enabled {
//                         write!(f, "{}", where_was.source_place())
//                     } else if CONFIG.is_show_github_source_place_enabled {
//                         write!(f, "{}", where_was.github_source_place())
//                     } else {
//                         write!(f, "")
//                     }
//                 }
//             },
//         }
//     }
// }

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
                    write!(f, "{}", self.source_place())
                }
                crate::config_mods::source_place_type::SourcePlaceType::Github => {
                    write!(f, "{}", self.github_source_place())
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
}
