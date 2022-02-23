use futures::future::join_all;
use strum::IntoEnumIterator;

use crate::helpers::where_was::WhereWas;

use crate::check_net::check_net_enum::CheckNet;

use super::check_net_enum::CheckNetError;

#[derive(Debug)]
pub struct CheckNetWrapperError {
    pub source: Vec<CheckNetError>,
    pub where_was: WhereWas,
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
                file: file!(),
                line: line!(),
                column: column!(),
            },
        }));
    }
    Ok(())
}
