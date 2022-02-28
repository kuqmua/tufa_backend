use actix_web::{get, web, Responder};

use crate::entry::entry;

#[get("/{id}/{name}/index.html")]
pub async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    entry();
    format!("Hello {}! id:{}", name, id)
}