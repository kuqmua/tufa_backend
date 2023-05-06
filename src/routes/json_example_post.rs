pub async fn json_example_post(json: actix_web::web::Json<tufa_common::json_example::JsonExample>) -> impl actix_web::Responder {
    println!("json example {:#?}", json);
    actix_web::HttpResponse::Ok().finish()
}
