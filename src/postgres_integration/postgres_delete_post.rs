use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::schema::providers_link_parts;

pub fn postgres_delete_post(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(providers_link_parts::table).execute(connection)
}
