use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::schema::posts;

pub fn postgres_delete_post(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(posts::table).execute(connection)
}
