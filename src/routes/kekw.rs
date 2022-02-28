use actix_web::{get, web, Responder};

#[get("/{kekw}/index.html")]
pub async fn kekw(web::Path(_): web::Path<String>) -> impl Responder {
    println!("kekw");
    format!("Hello kekw")
}