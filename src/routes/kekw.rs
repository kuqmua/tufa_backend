use actix_web::{get, web, Responder};

#[get("/{kekw}/index.html")]
async fn kekw(kekw: web::Path<String>) -> impl Responder {
    println!("kekw");
    format!("Hello {kekw}!")
}
