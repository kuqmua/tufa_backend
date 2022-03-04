use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};

#[get("/html/{name}")]
async fn html_route(name: web::Path<String>) -> impl Responder {
    println!("html_route");
    let f = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
        <head>
        <!-- This is equivalent to a HTTP header -->
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>title</title>
        </head>
        <!-- [...] -->
        <body>
        <div style="display: flex; justify-content: center; width: 100%; height: 100%;">
        <div style="background-color: red; width: 100px; height: 100px;">{name}</div>
        </div>
        </body>
        </html>"#
    );
    HttpResponse::Ok().content_type(ContentType::html()).body(f)
}
