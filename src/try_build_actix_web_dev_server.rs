//todo - make it async trait after async trait stabilization
pub async fn try_build_actix_web_dev_server<'a>(
    config: &'static (
        impl tufa_common::traits::config_fields::GetServerPort
        + tufa_common::traits::config_fields::GetHmacSecret
        + tufa_common::traits::config_fields::GetAccessControlMaxAge
        + tufa_common::traits::config_fields::GetAccessControlAllowOrigin
        + tufa_common::traits::get_email_client::GetEmailClient
        + tufa_common::traits::get_postgres_connect_options_with_db::GetPostgresConnectOptionsWithDb
        + tufa_common::traits::get_redis_url::GetRedisUrl
        + tufa_common::traits::get_server_address::GetServerAddress
        + tufa_common::traits::try_create_tcp_listener::TryCreateTcpListener<'a>
        + std::marker::Send 
        + std::marker::Sync
    ),
    postgres_connection_pool: sqlx::PgPool
) -> Result<actix_web::dev::Server, Box<tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::TryBuildActixWebDevServer<'a>>> {
    let tcp_listener = match config.try_create_tcp_listener() {
        Ok(tcp_listener) => tcp_listener,
        Err(e) => {
            return Err(Box::new(tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::TryBuildActixWebDevServer::TcpListenerBind {
                tcp_listener_bind: *e,
                code_occurence: tufa_common::code_occurence!(),
            }))
        },
    };
    let redis_session_store = match actix_session::storage::RedisSessionStore::new({
        use secrecy::ExposeSecret;
        config.get_redis_url().expose_secret()
    }).await {
        Ok(redis_session_store) => redis_session_store,
        Err(e) => {
            return Err(Box::new(tufa_common::repositories_types::tufa_server::try_build_actix_web_dev_server::TryBuildActixWebDevServer::NewRedisSessionStore {
                new_redis_session_store: e.to_string(),
                code_occurence: tufa_common::code_occurence!(),
            }))
        }
    };
    let server = match actix_web::HttpServer::new(move || {
        let secret_key = actix_web::cookie::Key::from({
            use secrecy::ExposeSecret;
            config.get_hmac_secret().expose_secret()
        }.as_bytes());
        actix_web::App::new()
            .wrap(actix_web_flash_messages::FlashMessagesFramework::builder(
                actix_web_flash_messages::storage::CookieMessageStore::builder(secret_key.clone()).build()
                )
                .build()
            )
            .wrap(actix_session::SessionMiddleware::new(
                redis_session_store.clone(),
                secret_key,
            ))
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(
                actix_cors::Cors::default()
                    .supports_credentials()
                    .allowed_origin(&config.get_access_control_allow_origin())//"http://127.0.0.1:8080"
                    .allow_any_method()
                    .allow_any_header()
                    .expose_any_header()
                    .max_age(*config.get_access_control_max_age()),
            ) //todo concrete host \ domain
            .route("/", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::home::home))
            .service(
                actix_web::web::scope("/admin")
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
                    .route("/json_example", actix_web::web::get().to(tufa_common::repositories_types::tufa_server::routes::json_example::json_example))
                    .route("/json_example_post", actix_web::web::post().to(tufa_common::repositories_types::tufa_server::routes::json_example_post::json_example_post))
                )
            )
            .route("/subscriptions", actix_web::web::post().to(crate::routes::subscribe))
            .route("/subscriptions/confirm", actix_web::web::get().to(crate::routes::confirm))
            .route("/newsletters", actix_web::web::post().to(crate::routes::publish_newsletter))
            .route(
                "/get_providers_posts",
                actix_web::web::post().to(tufa_common::repositories_types::tufa_server::routes::get_providers_posts_route::get_providers_posts_route),
            )
            .app_data(actix_web::web::Data::new(postgres_connection_pool.clone()))
            .app_data(actix_web::web::Data::new(config.get_email_client()))
            .app_data( actix_web::web::Data::new("localhost"))
            .app_data(actix_web::web::Data::new(config.get_hmac_secret()))
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