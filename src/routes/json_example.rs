use actix_web::{web, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct JsonExample {
    first: String,
    second: String,
}

pub async fn json_example() -> impl Responder {
    println!("json example");
    web::Json(JsonExample {
        first: "2".to_string(),
        second: "value2".to_string(),
    })
}
