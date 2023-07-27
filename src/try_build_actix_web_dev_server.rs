fn routes_static() -> axum::Router {
    axum::Router::new().nest_service(
        "/",
        axum::routing::get_service(tower_http::services::ServeDir::new("./")),
    )
}

async fn extract_custom_header_example(headers: http::header::HeaderMap) {
    let pc = headers.get("project_commit");
    println!("pc{pc:#?}")
}

async fn header_extractor_example(
    axum::TypedHeader(header): axum::TypedHeader<axum::headers::UserAgent>,
) {
    println!("header{:#?}", header);
}

async fn middleware_message_example(axum::Extension(shared_data): axum::Extension<SharedData>) {
    println!("message {}", shared_data.message);
}

#[derive(Clone)]
struct SharedData {
    pub message: std::string::String,
}

#[derive(Clone)] //or maybe add Clone to AppInfo too to solve possible problem?
struct HeaderMessage(pub std::string::String);

async fn read_middleware_custom_header(
    axum::Extension(message): axum::Extension<HeaderMessage>,
) -> std::string::String {
    println!("read_middleware_custom_header {}", message.0);
    message.0
}

pub async fn set_middleware_custom_header<B>(
    mut req: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<axum::response::Response, axum::http::StatusCode> {
    let request_project_commit = req
        .headers()
        .get(tufa_common::common::git::project_git_info::PROJECT_COMMIT)
        .ok_or_else(|| axum::http::StatusCode::BAD_REQUEST)?;
    let project_commit_checker_header = request_project_commit
        .to_str()
        .map_err(|_error| axum::http::StatusCode::BAD_REQUEST)?
        .to_owned();
    let extensions = req.extensions_mut();
    extensions.insert(HeaderMessage(project_commit_checker_header.to_owned()));
    Ok(next.run(req).await)
}

//todo - make it async trait after async trait stabilization
pub async fn try_build_actix_web_dev_server<'a>(
// tcp_listener: std::net::TcpListener,
    postgres_pool: sqlx::Pool<sqlx::Postgres>,
    redis_session_storage: actix_session::storage::RedisSessionStore,
    config: &'static tufa_common::repositories_types::tufa_server::config::config_struct::Config
) -> Result<actix_web::dev::Server, Box<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::TryBuildActixWebDevServer<'a>>>{
    println!(
        "server running on {}",
        tufa_common::common::config::get_server_address::GetServerAddress::get_server_address(
            &config
        )
    );
    let app_info = std::sync::Arc::new(
        tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo {
            postgres_pool,
            config,
            project_git_info:
                &tufa_common::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO,
            repository_git_info: &crate::global_variables::compile_time::git_info::GIT_INFO,
        },
    );
    let shared_data = SharedData {
        message: std::string::String::from("shared_message"),
    };
    axum::Server::bind(
        tufa_common::common::config::config_fields::GetSocketAddr::get_socket_addr(config),
    )
    .serve(
        axum::Router::new()
            .route(
                "/read_middleware_custom_header",
                axum::routing::get(read_middleware_custom_header),
            )
            .route_layer(axum::middleware::from_fn(set_middleware_custom_header))
            .nest("/api", crate::routes::api::cats::routes(app_info.clone()))
            .route(
                "/header_extractor_example",
                axum::routing::get(header_extractor_example),
            )
            .route(
                "/extract_custom_header_example",
                axum::routing::get(extract_custom_header_example),
            )
            .route(
                "/middleware_message_example",
                axum::routing::get(middleware_message_example),
            )
            .route_layer(axum::middleware::from_fn(
                tufa_common::server::middleware::project_commit_checker::project_commit_checker,
            ))
            .merge(tufa_common::server::routes::common_routes::common_routes(
                app_info,
            ))
            // .route_layer(axum::middleware::from_fn(
            //     tufa_common::server::middleware::content_type_application_json::content_type_application_json,
            // ))
            .fallback_service(routes_static()) //tufa_common::server::routes::not_found_route::fallback_service
            .layer(
                tower::ServiceBuilder::new().layer(tower_http::trace::TraceLayer::new_for_http()),
            )
            .layer(
                tower_http::cors::CorsLayer::new()
                    .allow_methods([
                        http::Method::GET,
                        http::Method::POST,
                        http::Method::PATCH,
                        http::Method::PUT,
                        http::Method::DELETE,
                    ])
                    .allow_origin([
                        "http://127.0.0.1".parse().unwrap(),
                        "http://localhost".parse().unwrap(),
                    ]),
            )
            .layer(axum::Extension(shared_data))
            .into_make_service(),
    )
    .await
    .unwrap_or_else(|e| panic!("axum builder serve await failed {e:#?}"));

    // let server = match actix_web::HttpServer::new(move || {
    //     let secret_key = actix_web::cookie::Key::from({
    //         use secrecy::ExposeSecret;
    //         use tufa_common::common::config::config_fields::GetHmacSecret;
    //         config.get_hmac_secret().expose_secret()
    //     }.as_bytes());
    //     actix_web::App::new()
    //     //todo - why no compile time error happens if you use
    //     .wrap(actix_web_flash_messages::FlashMessagesFramework::builder(
    //         actix_web_flash_messages::storage::CookieMessageStore::builder(secret_key.clone()).build()
    //         )
    //         .build()
    //     )
    //     .wrap(actix_session::SessionMiddleware::new(
    //         redis_session_storage.clone(),
    //         secret_key,
    //     ))
    //     .wrap(tracing_actix_web::TracingLogger::default())
    //     .wrap(
    //         actix_cors::Cors::default()
    //             .supports_credentials()
    //             // .allowed_origin(&config.get_access_control_allow_origin())
    //             .allow_any_origin()
    //             .allow_any_method()
    //             .allow_any_header()
    //             .expose_any_header()
    //             .max_age({
    //             use tufa_common::common::config::config_fields::GetAccessControlMaxAge;
    //             *config.get_access_control_max_age()
    //             }),
    //     )
    //     .app_data(actix_web::web::Data::new({
    //         use tufa_common::common::config::get_email_client::GetEmailClient;
    //         config.get_email_client()
    //     }))
    //     .app_data(actix_web::web::Data::new({
    //         use tufa_common::common::config::config_fields::GetHmacSecret;
    //         config.get_hmac_secret().clone()
    //     }))
    //     .app_data(actix_web::web::Data::new(tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::AppInfo {
    //         postgres_pool: postgres_pool.clone(),//if use it without .clone() - will be runtime error if you try to reach route
    //         config: config,
    //         project_git_info: &tufa_common::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO,
    //         repository_git_info: &crate::global_variables::compile_time::git_info::GIT_INFO,
    //     }))
    //     //todo - service capabilities ?
    //     .service(
    //         actix_web::web::scope("/admin")
    //             .guard(actix_web::guard::Host("127.0.0.1"))
    //             .wrap(actix_web_lab::middleware::from_fn(tufa_common::repositories_types::tufa_server::authentication::reject_anonymous_users))
    //             .route("/dashboard", actix_web::web::get().to(crate::routes::dashboard::admin_dashboard))
    //             // .route("/newsletters", web::get().to(tufa_common::repositories_types::tufa_server::routes::publish_newsletter_form))
    //             .route("/newsletters", actix_web::web::post().to(crate::routes::publish_newsletter))
    //             .route("/password", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::admin::change_password_form))
    //             .route("/password", actix_web::web::post().to(crate::routes::admin::password::change_password))
    //             .route("/logout", actix_web::web::post().to(tufa_common::repositories_types::tufa_server::routes::admin::log_out)),
    //     )
    //     .route("/login", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::login::login_form))
    //     .route("/login", actix_web::web::post().to(crate::routes::login::login))
    //     .route("/subscriptions", actix_web::web::post().to(crate::routes::subscribe))
    //     .route("/subscriptions/confirm", actix_web::web::get().to(crate::routes::confirm))
    //     .route("/newsletters", actix_web::web::post().to(crate::routes::publish_newsletter))
    //     //
    //     .route("/health_check", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::health_check))
    //     .service(
    //         actix_web::web::scope("/api")
    //         .service(tufa_common::server::routes::git_info::git_info)
    //         .service(
    //         // actix_web::web::resource("/cats")
    //             actix_web::web::scope(&format!("/{}", tufa_common::repositories_types::tufa_server::routes::api::cats::CATS))
    //             // .guard(actix_web::guard::Host("www.rust-lang.org"))
    //             .service(crate::routes::api::cats::get)
    //             .service(crate::routes::api::cats::get_by_id)
    //             .service(crate::routes::api::cats::post)
    //             .service(crate::routes::api::cats::put)
    //             .service(crate::routes::api::cats::patch)
    //             .service(crate::routes::api::cats::delete)
    //             .service(crate::routes::api::cats::delete_by_id)
    //         )
    //     )

    // })
    // .listen(tcp_listener)
    // {
    //     Ok(server) => server,
    //     Err(e) => {
    //         return Err(Box::new(tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::TryBuildActixWebDevServer::HttpServerListen {
    //             http_server_listen: e,
    //             code_occurence: tufa_common::code_occurence!(),
    //         }))
    //     }
    // }
    // .run();
    // Ok(server)
    todo!()
}
