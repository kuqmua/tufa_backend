use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::postgres_integration::schema::posts;

use crate::postgres_integration::models::{NewPost, Post};

pub fn postgres_create_post<'a>(
    connection: &PgConnection,
    title: &'a str,
    body: &'a str,
) -> Result<Post, diesel::result::Error> {
    diesel::insert_into(posts::table)
        .values(&NewPost { title, body })
        .get_result(connection)
}
