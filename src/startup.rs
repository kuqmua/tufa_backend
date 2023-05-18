pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build<'a>(settings: tufa_common::repositories_types::tufa_server::settings::Settings) -> Result<Self, Box<tufa_common::repositories_types::tufa_server::startup::ApplicationBuildErrorNamed<'a>>> {
        let connection_pool = tufa_common::repositories_types::tufa_server::startup::get_connection_pool(&settings.database);
        let listener = match std::net::TcpListener::bind(&format!(
            "localhost:{}",
            settings.application.port
        )) {
            Ok(listener) => listener,
            Err(e) => {
                return Err(Box::new(tufa_common::repositories_types::tufa_server::startup::ApplicationBuildErrorNamed::TcpListenerBind {
                    tcp_listener_bind: e,
                    code_occurence: tufa_common::code_occurence!(),
                }))
            }
        };
        let port = match listener.local_addr() {
            Ok(address) => address,
            Err(e) => {
                return Err(Box::new(
                    tufa_common::repositories_types::tufa_server::startup::ApplicationBuildErrorNamed::TcpListenerLocalAddress { 
                        tcp_listener_local_address: e,
                        code_occurence: tufa_common::code_occurence!(),
                    },
                ))
            }
        }
        .port();
        let server = match run(
            listener,
            connection_pool,
            settings.email_client.client(),
            settings.application.hmac_secret,
            settings.redis_uri,
        )
        .await
        {
            Ok(server) => server,
            Err(e) => {
                return Err(Box::new(tufa_common::repositories_types::tufa_server::startup::ApplicationBuildErrorNamed::ApplicationRun {
                    application_run: *e,
                    code_occurence: tufa_common::code_occurence!(),
                }))
            }
        };
        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped<'a>(self) -> Result<(), tufa_common::repositories_types::tufa_server::startup::RunUntilStoppedErrorNamed<'a>> {
        match self.server.await {
            Err(e) => Err(tufa_common::repositories_types::tufa_server::startup::RunUntilStoppedErrorNamed::RunUntilStopped {
                run_until_stopped: e,
                code_occurence: tufa_common::code_occurence!(),
            }),
            Ok(_) => Ok(()),
        }
    }
}

async fn run<'a>(
    listener: std::net::TcpListener,
    db_pool: sqlx::PgPool,
    email_client: tufa_common::repositories_types::tufa_server::email_client::EmailClient,
    hmac_secret: secrecy::Secret<String>,
    redis_uri: secrecy::Secret<String>,
) -> Result<actix_web::dev::Server, Box<tufa_common::repositories_types::tufa_server::startup::ApplicationRunErrorNamed<'a>>> {
    let data_db_pool = actix_web::web::Data::new(db_pool);
    let email_client = actix_web::web::Data::new(email_client);
    let secret_key = actix_web::cookie::Key::from({
        use secrecy::ExposeSecret;
        hmac_secret.expose_secret()
    }.as_bytes());
    let message_store = actix_web_flash_messages::storage::CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = actix_web_flash_messages::FlashMessagesFramework::builder(message_store).build();
    let redis_store = match actix_session::storage::RedisSessionStore::new({
        use secrecy::ExposeSecret;
        redis_uri.expose_secret()
    }).await {
        Ok(redis_session_store) => redis_session_store,
        Err(e) => {
            return Err(Box::new(tufa_common::repositories_types::tufa_server::startup::ApplicationRunErrorNamed::NewRedisSessionStore {
                new_redis_session_store: e.to_string(),
                code_occurence: tufa_common::code_occurence!(),
            }))
        }
    };
    let server = match actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(message_framework.clone())
            .wrap(actix_session::SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(
                actix_cors::Cors::default()
                    .supports_credentials()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allow_any_method()
                    .allow_any_header()
                    .expose_any_header()
                    .max_age(3600),
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
            .app_data(data_db_pool.clone())
            .app_data(email_client.clone())
            .app_data( actix_web::web::Data::new("localhost"))
            .app_data(actix_web::web::Data::new(hmac_secret.clone()))
    })
    .listen(listener)
    {
        Ok(server) => server,
        Err(e) => {
            return Err(Box::new(tufa_common::repositories_types::tufa_server::startup::ApplicationRunErrorNamed::HttpServerListen {
                http_server_listen: e,
                code_occurence: tufa_common::code_occurence!(),
            }))
        }
    }
    .run();
    Ok(server)
}
