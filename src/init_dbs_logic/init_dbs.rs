use futures::future::join_all;
use strum::IntoEnumIterator;

use crate::init_dbs_logic::init_tables_enum::InitTablesEnum;

use crate::init_dbs_logic::init_tables_enum::InitDbsErrorEnum;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn init_dbs() -> Result<(), Vec<Box<InitDbsErrorEnum>>> {
    let results = join_all(InitTablesEnum::iter().map(|table| async move { table.init().await }))
        .await
        .into_iter()
        .filter_map(|result| {
            if let Err(e) = result {
                return Some(e);
            }
            None
        })
        .collect::<Vec<Box<InitDbsErrorEnum>>>();
    if !results.is_empty() {
        return Err(results);
    }
    Ok(())
}
