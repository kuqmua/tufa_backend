use crate::helpers::get_server_address::get_server_address;
use crate::routes::get_providers_posts_route::get_providers_posts_route;
use actix_web::App;
use actix_web::HttpServer;

#[actix_web::main] // or #[tokio::main]
pub async fn server_wrapper() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(|| async { "Hello World!" }))
            .service(get_providers_posts_route)
    })
    .bind(get_server_address())?
    .run()
    .await
}
