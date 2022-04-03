use actix_web::http::header::LOCATION;
use actix_web::{post, HttpResponse};
// use secrecy::Secret;

// #[derive(serde::Deserialize)]
// pub struct FormData {
//     username: String,
//     password: Secret<String>,
// }

#[post("/login")]
pub async fn login() -> HttpResponse {
    //form: web::Form<FormData>
    println!("login");
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
