use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::schemas::github_link_parts_schema::github_link_parts;

pub fn postgres_github_delete_post(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(github_link_parts::table).execute(connection)
}