//todo - make it async trait after async trait stabilization
pub async fn try_build_actix_web_dev_server<'a>(
    tcp_listener: std::net::TcpListener,
    postgres_pool: sqlx::Pool<sqlx::Postgres>,
    redis_session_storage: actix_session::storage::RedisSessionStore,
    config: &'static tufa_common::repositories_types::tufa_server::config::config_struct::Config
) -> Result<actix_web::dev::Server, Box<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::TryBuildActixWebDevServer<'a>>> {

// Shared Mutable State
// use actix_web::{web, App, HttpServer};
// use std::sync::Mutex;

// struct AppStateWithCounter {
//     counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
// }

// async fn index(data: web::Data<AppStateWithCounter>) -> String {
//     let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
//     *counter += 1; // <- access counter inside MutexGuard

//     format!("Request number: {counter}") // <- response with count
// }

// use actix_web::{web, App, HttpResponse, HttpServer};

// // this function could be located in a different module
// fn scoped_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/test")
//             .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
//             .route(web::head().to(HttpResponse::MethodNotAllowed)),
//     );
// }

// // this function could be located in a different module
// fn config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/app")
//             .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
//             .route(web::head().to(HttpResponse::MethodNotAllowed)),
//     );
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .configure(config)
//             .service(web::scope("/api").configure(scoped_config))
//             .route(
//                 "/",
//                 web::get().to(|| async { HttpResponse::Ok().body("/") }),
//             )
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

    let server = match actix_web::HttpServer::new(move || {
        let secret_key = actix_web::cookie::Key::from({
            use secrecy::ExposeSecret;
            use tufa_common::common::config::config_fields::GetHmacSecret;
            config.get_hmac_secret().expose_secret()
        }.as_bytes());
            actix_web::App::new()
            .wrap(actix_web_flash_messages::FlashMessagesFramework::builder(
                actix_web_flash_messages::storage::CookieMessageStore::builder(secret_key.clone()).build()
                )
                .build()
            )
            .wrap(actix_session::SessionMiddleware::new(
                redis_session_storage.clone(),
                secret_key,
            ))
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(
                actix_cors::Cors::default()
                    .supports_credentials()
                    // .allowed_origin(&config.get_access_control_allow_origin())
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .expose_any_header()
                    .max_age({
                        use tufa_common::common::config::config_fields::GetAccessControlMaxAge;
                        *config.get_access_control_max_age()
                    }),
            ) //todo concrete host \ domain
            .route("/", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::home::home))
            .service(
                actix_web::web::scope("/admin")
                    .guard(actix_web::guard::Host("127.0.0.1"))
                    .wrap(actix_web_lab::middleware::from_fn(tufa_common::repositories_types::tufa_server::authentication::reject_anonymous_users))
                    .route("/dashboard", actix_web::web::get().to(crate::routes::dashboard::admin_dashboard))
                    // .route("/newsletters", web::get().to(tufa_common::repositories_types::tufa_server::routes::publish_newsletter_form))
                    .route("/newsletters", actix_web::web::post().to(crate::routes::publish_newsletter))
                    .route("/password", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::change_password_form))
                    .route("/password", actix_web::web::post().to(crate::routes::admin::password::change_password))
                    .route("/logout", actix_web::web::post().to(tufa_common::repositories_types::tufa_server::routes::log_out)),
            )
            .route("/login", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::login::login_form))
            .route("/login", actix_web::web::post().to(crate::routes::login::login))
            .route("/health_check", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::health_check))
            .service(
                actix_web::web::scope("/api")
                .service(
                    actix_web::web::scope("/html")//or maybe .md ?
                    .route("/git_info", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::git::git_info_html::git_info_html))
                )
                .service(
                    actix_web::web::scope("/json")
                    .route("/git_info", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::git::git_info_json::git_info_json))
                )
                .service(
                // actix_web::web::resource("/cats")
                    actix_web::web::scope("/cats")
                    .route("/", actix_web::web::get().to(crate::routes::cats::get_all))
                    // .route(actix_web::web::post().to(crate::routes::cats::post))
                    .service(
                        crate::routes::cats::select_cats
                    )
                )
            )
            .route("/subscriptions", actix_web::web::post().to(crate::routes::subscribe))
            .route("/subscriptions/confirm", actix_web::web::get().to(crate::routes::confirm))
            .route("/newsletters", actix_web::web::post().to(crate::routes::publish_newsletter))
            .route(
                "/get_providers_posts",
                actix_web::web::post().to(tufa_common::repositories_types::tufa_server::routes::get_providers_posts_route::get_providers_posts_route),
            )
            .app_data(actix_web::web::Data::new(postgres_pool.clone()))//if use it without .clone() - will be runtime error if you try to reach route
            .app_data(actix_web::web::Data::new({
                use tufa_common::common::config::get_email_client::GetEmailClient;
                config.get_email_client()
            }))
            .app_data(actix_web::web::Data::new({
                use tufa_common::common::config::config_fields::GetHmacSecret;
                config.get_hmac_secret().clone()
            }))
            .app_data(actix_web::web::Data::new(config))
        
    })
    .listen(tcp_listener)
    {
        Ok(server) => server,
        Err(e) => {
            return Err(Box::new(tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::TryBuildActixWebDevServer::HttpServerListen {
                http_server_listen: e,
                code_occurence: tufa_common::code_occurence!(),
            }))
        }
    }
    .run();
    Ok(server)
}