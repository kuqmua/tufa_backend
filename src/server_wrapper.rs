use crate::helpers::get_server_address::get_server_address;
use crate::routes::default::default_route::default_route;
use crate::routes::get_providers_posts_route::get_providers_posts_route;
use crate::routes::login::login_form::login_form;
use crate::routes::login::login_handle::login_handle;
use actix_web::App;
use actix_web::HttpServer;

#[actix_web::main] // or #[tokio::main]
pub async fn server_wrapper() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(|| async { "Hello World!" }))
            .service(get_providers_posts_route)
            .service(login_form)
            .service(login_handle)
            .service(default_route)
        // .service(manual_hello)
    })
    .bind(get_server_address())?
    .run()
    .await
}
