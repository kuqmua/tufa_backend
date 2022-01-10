use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::schemas::biorxiv_link_parts_schema::biorxiv_link_parts;

pub fn postgres_biorxiv_delete_post(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(biorxiv_link_parts::table).execute(connection)
}