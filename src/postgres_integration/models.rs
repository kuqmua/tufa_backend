#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use super::schema::posts;

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
        title: &'a str,
        body: &'a str,
    ) -> Result<Post, diesel::result::Error> {
        diesel::insert_into(posts::table)
            .values(&NewPost { title, body })
            .get_result(connection)
    }
}
