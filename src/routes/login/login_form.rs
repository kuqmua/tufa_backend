use actix_web::{get, HttpResponse};
use actix_web::http::header::ContentType;

#[get("/login_form/")]
pub async fn login_form() -> HttpResponse {
    println!("login_form");
    //redirect into /login clicking on submit button
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login.html"))
}