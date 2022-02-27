use std::fmt;

use chrono::{DateTime, FixedOffset, Local, Utc};

use futures::future::join_all;
use strum::IntoEnumIterator;

use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;

use crate::check_net::check_net_enum::CheckNet;

use super::check_net_enum::CheckNetError;

use crate::check_net::check_net_availability::CheckNetAvailabilityErrorEnum;

use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityErrorEnum;

#[derive(Debug)]
pub struct CheckNetWrapperError {
    pub source: Vec<CheckNetError>,
    pub where_was: WhereWas,
}

impl fmt::Display for CheckNetWrapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stringified_errors = self
            .source
            .iter()
            .map(|e| format!("{e}\n"))
            .collect::<String>();
        if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled {
            write!(
                f,
                "{}\n{}\n{}",
                self.where_was.source_place_with_readable_time(),
                self.where_was.github_source_place_with_readable_time(),
                stringified_errors
            )
        } else if CONFIG.is_show_source_place_enabled {
            write!(
                f,
                "{}\n{}",
                self.where_was.source_place_with_readable_time(),
                stringified_errors
            )
        } else if CONFIG.is_show_github_source_place_enabled {
            write!(
                f,
                "{}\n{}",
                self.where_was.github_source_place_with_readable_time(),
                stringified_errors
            )
        } else {
            write!(f, "{}", stringified_errors)
        }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn check_net_wrapper() -> Result<(), Box<CheckNetWrapperError>> {
    let err_vec = join_all(CheckNet::iter().map(|v| async move { v.check().await }))
        .await
        .into_iter()
        .filter_map(|r| {
            if let Err(e) = r {
                return Some(e);
            }
            None
        })
        .collect::<Vec<CheckNetError>>();
    if !err_vec.is_empty() {
        return Err(Box::new(CheckNetWrapperError {
            source: err_vec,
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
