

use actix_web::{get, web, Responder};

#[get("/html/{name}/index.html")]
async fn html_route(name: web::Path<String>) -> impl Responder {
    println!("html_route");
    format!("todo html {name}")
}