use actix_web::{get, web, Responder};

use crate::providers::get_providers_posts::get_providers_posts;

#[get("/fff/{name}")]
async fn index(name: web::Path<String>) -> impl Responder {
    get_providers_posts().await;
    format!("Hello {name}!")
}