use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::schemas::medrxiv_link_parts_schema::medrxiv_link_parts;

pub fn postgres_medrxiv_delete_post(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(medrxiv_link_parts::table).execute(connection)
}