use actix_web::{get, HttpResponse, Responder};

#[get("/get_echo")]
async fn get_echo(req_body: String) -> impl Responder {
    println!("get_echo");
    HttpResponse::Ok().body(req_body)
}