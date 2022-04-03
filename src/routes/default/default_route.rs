use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn default_route() -> HttpResponse {
    println!("default_route");
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("home.html"))
}
