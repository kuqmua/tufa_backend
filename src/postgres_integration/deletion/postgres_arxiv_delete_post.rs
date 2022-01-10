use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::schemas::arxiv_link_parts_schema::arxiv_link_parts;

pub fn postgres_arxiv_delete_post(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(arxiv_link_parts::table).execute(connection)
}
