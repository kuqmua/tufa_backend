pub mod establish_connection;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use schema::posts;

use self::models::{NewPost, Post};

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Option<Post> {
    let new_post = NewPost { title, body };
    let result: Result<Post, diesel::result::Error> = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn);
    match result {
        Ok(post) => Some(post),
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "diesel::insert_into .values .get_result {} error: {:#?}",
                    &title, e
                ),
            );
            None
        }
    }
}
