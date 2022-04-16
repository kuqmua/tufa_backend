use actix_web::{web, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct JsonExample {
    first: String,
    second: String,
}

pub async fn json_example() -> impl Responder {
    println!("json example");
    let mut vec: Vec<JsonExample> = Vec::new();
    vec.push(JsonExample {
        first: "1".to_string(),
        second: "value1".to_string(),
    });
    vec.push(JsonExample {
        first: "2".to_string(),
        second: "value2".to_string(),
    });
    web::Json(vec)
}
