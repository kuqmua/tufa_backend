use actix_web::{post, HttpResponse};
use actix_web::http::header::LOCATION;

#[post("/login")]
pub async fn login() -> HttpResponse {
    println!("login");
    HttpResponse::Ok()
    .insert_header((LOCATION, "/"))
    .finish()
}