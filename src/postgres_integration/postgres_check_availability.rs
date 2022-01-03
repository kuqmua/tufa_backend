use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::check_net::check_net_error_enum::CheckNetError;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn postgres_check_availability(postgres_url: &str) -> Result<(), Box<CheckNetError>> {
    if let Err(e) = PgConnection::establish(postgres_url) {
        return Err(Box::new(CheckNetError::Postgres { error: Box::new(e) }));
    }
    Ok(())
}
