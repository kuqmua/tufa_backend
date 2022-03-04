use actix_web::App;
use actix_web::HttpServer;

use crate::helpers::get_server_address::get_server_address;

use crate::routes::default_route::default_route;
use crate::routes::get_providers_posts_route::get_providers_posts_route;
use crate::routes::html::html_route::html_route;
use crate::routes::html::html_route_from_file::html_route_from_file;
use crate::routes::kekw::kekw;
use crate::routes::post_echo::post_echo;
use crate::routes::login::login_form::login_form;

#[actix_web::main] // or #[tokio::main]
pub async fn server_wrapper() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(|| async { "Hello World!" }))
            .service(get_providers_posts_route)
            .service(kekw)
            .service(default_route)
            .service(post_echo)
            .service(html_route)
            .service(html_route_from_file)
            .service(login_form)
        // .service(manual_hello)
    })
    .bind(get_server_address())?
    .run()
    .await
}
