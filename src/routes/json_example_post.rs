use actix_web::{web, Responder};
use tufa_common::json_example::JsonExample;

pub async fn json_example_post(json: web::Json<JsonExample>) -> impl Responder {
    println!("json example {:#?}", json);
    web::Json(JsonExample {
        first: "first_value_json_example".to_string(),
        second: "second_value_json_example".to_string(),
    })
}
