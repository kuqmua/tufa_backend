use crate::configuration::{DatabaseSettings, EmailClientSettings};
use crate::email_client::EmailClient;
// use crate::domain::SubscriberEmail;
// use crate::authentication::reject_anonymous_users;
// use crate::configuration::{DatabaseSettings, Settings};
// use crate::email_client::EmailClient;
// use crate::routes::admin_dashboard;
// use crate::routes::change_password;
// use crate::routes::change_password_form;
// use crate::routes::confirm;
// use crate::routes::health_check;
// use crate::routes::log_out;
use crate::routes::login::login;
use crate::routes::login::login_form;
// use crate::routes::publish_newsletter;
// use crate::routes::publish_newsletter_form;
// use crate::routes::subscribe;
use crate::routes::home::home;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_lab::middleware::from_fn;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use crate::config_mods::lazy_static_config::CONFIG;
use crate::domain::SubscriberEmail;

pub struct Application {
    port: u16,
    server: Server,
}

pub enum ApplicationBuildErrorEnum {
    TcpListenerBind {
        source: std::io::Error,
    },
    TcpListenerLocalAddress {
        source: std::io::Error,
    },
    ApplicationRun {
        source: Box<ApplicationRunErrorEnum>
    }
}

impl Application {
    pub async fn build() -> Result<Self, Box<ApplicationBuildErrorEnum>> {
        let db = DatabaseSettings {
            host: &CONFIG.postgres_ip,
            port: CONFIG.postgres_port,
            username: &CONFIG.postgres_login,
            password: Secret::new(CONFIG.postgres_password.clone()),
            database_name: &CONFIG.postgres_db,
            require_ssl: CONFIG.require_ssl,
        };
        let connection_pool = get_connection_pool(db.with_db());
        let email_client = EmailClientSettings {
            base_url: CONFIG.base_url.clone(),
            sender_email: "test@gmail.com".to_string(),
            authorization_token: Secret::new("my-secret-token".to_string()),
            timeout_milliseconds: 10000,
        }.client();
        let listener = match TcpListener::bind(&format!(
            "{}:{}",
            CONFIG.server_ip, CONFIG.server_port
        )) {
            Ok(listener) => listener,
            Err(e) => return Err(Box::new(ApplicationBuildErrorEnum::TcpListenerBind {
                source: e
            })),
        };
        let port = match listener.local_addr() {
            Ok(address) => address,
            Err(e) => return Err(Box::new(ApplicationBuildErrorEnum::TcpListenerLocalAddress  {
                source: e
            })),
        }.port();
        let server = match run(
            listener,
            connection_pool,
            email_client,
            format!("http://{}", CONFIG.server_ip),//configuration.application.base_url,
            Secret::new("super-long-and-secret-random-key-needed-to-verify-message-integrity".to_string()), //"configuration.application.hmac_secret,
            Secret::new("redis://127.0.0.1:6379".to_string())//configuration.redis_uri,
        )
        .await {
            Ok(server) => server,
            Err(e) => return Err(Box::new(ApplicationBuildErrorEnum::ApplicationRun {
                source: e
            })),
        };
        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(pg_connection_options: PgConnectOptions) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(pg_connection_options)
}

pub struct ApplicationBaseUrl(pub String);

pub enum ApplicationRunErrorEnum {
    NewRedisSessionStore {
        source: anyhow::Error,
    },
    HttpServerListen {
        source: std::io::Error,
    }
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
) -> Result<Server, Box<ApplicationRunErrorEnum>> {
    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = match RedisSessionStore::new(redis_uri.expose_secret()).await {
        Ok(redis_session_store) => redis_session_store,
        Err(e) => return Err(Box::new(ApplicationRunErrorEnum::NewRedisSessionStore {
            source: e
        })),
    };
    let server = match HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            // .service(
            //     web::scope("/admin")
            //         .wrap(from_fn(reject_anonymous_users))
            //         .route("/dashboard", web::get().to(admin_dashboard))
            //         .route("/newsletters", web::get().to(publish_newsletter_form))
            //         .route("/newsletters", web::post().to(publish_newsletter))
            //         .route("/password", web::get().to(change_password_form))
            //         .route("/password", web::post().to(change_password))
            //         .route("/logout", web::post().to(log_out)),
            // )
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            // .route("/health_check", web::get().to(health_check))
            // .route("/subscriptions", web::post().to(subscribe))
            // .route("/subscriptions/confirm", web::get().to(confirm))
            // .route("/newsletters", web::post().to(publish_newsletter))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
    })
    .listen(listener) {
        Ok(server) => server,
        Err(e) => return Err(Box::new(ApplicationRunErrorEnum::HttpServerListen {
            source: e
        })),
    }
    .run();
    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);
