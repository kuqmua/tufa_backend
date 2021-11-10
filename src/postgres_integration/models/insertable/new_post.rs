use crate::postgres_integration::schema::posts;
use crate::postgres_integration::models::queryable::post::Post;

use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

impl<'a> NewPost<'a> {
    pub fn insert_into_postgres(
        connection: &PgConnection,
        new_post: Self,
    ) -> Result<Post, diesel::result::Error> {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(connection)
    }
}
