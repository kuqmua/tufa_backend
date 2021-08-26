pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

pub fn establish_connection(database_url: String) -> Option<PgConnection> {
    let dotenv_result = dotenv();
    match dotenv_result {
        Ok(_) => {
            let result_establish_connection = PgConnection::establish(&database_url);
            match result_establish_connection {
                Ok(pg_connection) => Some(pg_connection),
                Err(e) => {
                    print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        file!().to_string(),
                        line!().to_string(),
                        format!("PgConnection::establish {} error: {:#?}", &database_url, e),
                    );
                    None
                }
            }
        }
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!("dotenv error: {:#?}", e),
            );
            None
        }
    }
}

use self::models::{NewPost, Post};

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
