use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::prelude::ConnectionError;

use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn postgres_check_availability() -> Result<(), ConnectionError> {
    PgConnection::establish(&postgres_get_db_url())?;
    Ok(())
}
