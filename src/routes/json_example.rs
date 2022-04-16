use actix_web::{web, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Country {
    first: String,
    second: String,
}

pub async fn json_example() -> impl Responder {
    println!("json example");
    let mut vec: Vec<Country> = Vec::new();
    vec.push(Country {
        first: "1".to_string(),
        second: "value1".to_string(),
    });
    vec.push(Country {
        first: "2".to_string(),
        second: "value2".to_string(),
    });
    return web::Json(vec);
}
