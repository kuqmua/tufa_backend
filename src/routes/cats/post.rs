// curl -X POST http://127.0.0.1:8080/api/cats -H 'Content-Type: application/json' -d '[{"id":1,"name":"black"}]'
pub async fn post(
    cat: actix_web::web::Json<Vec<tufa_common::repositories_types::tufa_server::routes::cats::get::Cats>>
) -> impl actix_web::Responder {
    println!("posted cat\n{:#?}", cat);
    actix_web::HttpResponse::Ok().finish()
}