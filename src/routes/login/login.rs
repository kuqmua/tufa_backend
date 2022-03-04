use actix_web::{post, HttpResponse};

#[post("/login")]
pub async fn login() -> HttpResponse {
    println!("login");
    HttpResponse::Ok().finish()
}