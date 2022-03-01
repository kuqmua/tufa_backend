use actix_web::{get, web, Responder};

use crate::entry::entry;

#[get("/fff/{name}")]
async fn index(name: web::Path<String>) -> impl Responder {
    entry().await;
    format!("Hello {name}!")
}