use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::schemas::habr_link_parts_schema::habr_link_parts;

pub fn postgres_habr_delete_post(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(habr_link_parts::table).execute(connection)
}