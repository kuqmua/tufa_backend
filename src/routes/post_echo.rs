use actix_web::{post, HttpResponse, Responder};

#[post("/echo")]
async fn post_echo(req_body: String) -> impl Responder {
    println!("echo");
    HttpResponse::Ok().body(req_body)
}