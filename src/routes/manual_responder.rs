use actix_web::{Responder, HttpResponse};

pub async fn manual_responder() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}