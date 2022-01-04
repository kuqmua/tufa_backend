use diesel::pg::PgConnection;
use diesel::prelude::*;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn postgres_check_availability(postgres_url: &str) -> Result<(), Box<ConnectionError>> {
    if let Err(e) = PgConnection::establish(postgres_url) {
        return Err(Box::new(e));
    }
    Ok(())
}
