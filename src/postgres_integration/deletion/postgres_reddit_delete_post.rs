use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::schemas::reddit_link_parts_schema::reddit_link_parts;

pub fn postgres_reddit_delete_post(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(reddit_link_parts::table).execute(connection)
}