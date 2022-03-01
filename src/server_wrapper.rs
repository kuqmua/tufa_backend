use actix_web::App;
use actix_web::HttpServer;

use crate::helpers::get_server_address::get_server_address;

use crate::routes::index::index;
use crate::routes::kekw::kekw;
use crate::routes::default_route::default_route;
use crate::routes::post_echo::post_echo;

#[actix_web::main] // or #[tokio::main]
pub async fn server_wrapper() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(|| async { "Hello World!" }))
            .service(index)
            .service(kekw)
            .service(default_route)
            .service(post_echo)
            // .service(manual_hello)
    })
    .bind(get_server_address())?
    .run()
    .await
}