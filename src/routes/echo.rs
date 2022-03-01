use actix_web::{post, HttpResponse, Responder};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("echo");
    HttpResponse::Ok().body(req_body)
}