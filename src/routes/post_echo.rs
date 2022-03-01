use actix_web::{post, HttpResponse, Responder};

#[post("/post_echo")]
async fn post_echo(req_body: String) -> impl Responder {
    println!("post_echo");
    HttpResponse::Ok().body(req_body)
}