use actix_web::{get, web, Responder};

use crate::entry::entry;

#[get("/{name}")]
async fn index(name: web::Path<String>) -> impl Responder {
    entry();
    format!("Hello {name}!")
}