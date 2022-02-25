use std::fmt;

use chrono::{DateTime, FixedOffset, Local, Utc};

use futures::future::join_all;
use strum::IntoEnumIterator;

use crate::helpers::where_was::WhereWas;

use crate::check_net::check_net_enum::CheckNet;
use crate::traits::where_was_trait::WhereWasTrait;

use super::check_net_enum::CheckNetError;

use crate::check_net::check_net_availability::CheckNetAvailabilityErrorEnum;

#[derive(Debug)]
pub struct CheckNetWrapperError {
    pub source: Vec<CheckNetError>,
    pub where_was: WhereWas,
}

// CheckNetWrapperError {
//     source: [
//         Postgres(
//             PostgresCheckAvailabilityError {
//                 source: PoolTimedOut,
//                 where_was: WhereWas {
//                     file: "src/postgres_integration/postgres_check_availability.rs",
//                     line: 34,
//                     column: 25,
//                 },
//             },
//         ),
//         Mongo(
//             ClientOptionsParse {
//                 source: Error {
//                     kind: InvalidArgument {
//                         message: "invalid connection string scheme: postgres",
//                     },
//                     labels: {},
//                 },
//                 where_was: WhereWas {
//                     file: "src/mongo_integration/mongo_check_availability.rs",
//                     line: 40,
//                     column: 29,
//                 },
//             },
//         ),
//     ],
//     where_was: WhereWas {
//         file: "src/check_net/check_net_wrapper.rs",
//         line: 38,
//         column: 25,
//     },
// }
impl fmt::Display for CheckNetWrapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // enum Something {
        //     TODO! how to write from WhereWas conversion properly?
        // }
        // for i in &self.source {
        //     match i {
        //         CheckNetError::Net(e) => {
        //             match e {
        //                 CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError { source, where_was } => {

        //                 },
        //                 CheckNetAvailabilityErrorEnum::StatusCodeError { source, where_was } => {

        //                 },
        //             }
        //         },
        //         CheckNetError::Postgres(e) => {

        //         },
        //         CheckNetError::Mongo(e) => {

        //         },
        //     }
        // }
        write!(
            f,
            "{}\n{})",
            self.where_was.source_place(),
            self.where_was.github_source_place()
        )
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
