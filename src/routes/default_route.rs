use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn default_route() -> impl Responder {
    println!("default_route");
    HttpResponse::Ok().body("default_route")
}
