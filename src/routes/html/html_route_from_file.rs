use actix_web::{get, http::header::ContentType, HttpResponse, Responder};

#[get("/html_from_file/")]
async fn html_route_from_file() -> impl Responder {
    println!("html_route_from_file");
    HttpResponse::Ok().content_type(ContentType::html()).body(include_str!("home.html"))
}