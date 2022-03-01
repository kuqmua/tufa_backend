use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    println!("hello");
    HttpResponse::Ok().body("hello")
}