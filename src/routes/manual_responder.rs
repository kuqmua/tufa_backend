use actix_web::{HttpResponse, Responder};

pub async fn manual_responder() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
