use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::prelude::ConnectionError;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn postgres_check_availability(postgres_url: &str) -> Result<(), ConnectionError> {
    PgConnection::establish(postgres_url)?;//todo set timeout for this function
    Ok(())
}
