use actix_web::{get, HttpResponse, Responder};

#[get("/get_echo")]
async fn get_echo() -> impl Responder {
    println!("get_echo");
    HttpResponse::Ok().body("get_echo")
}
