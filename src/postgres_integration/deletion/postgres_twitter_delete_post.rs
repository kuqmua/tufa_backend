use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::schemas::twitter_link_parts_schema::twitter_link_parts;

pub fn postgres_twitter_delete_post(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(twitter_link_parts::table).execute(connection)
}