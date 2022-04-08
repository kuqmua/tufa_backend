use crate::config_mods::lazy_static_config::CONFIG;
use crate::configuration;
use crate::configuration::ApplicationSettings;
use crate::configuration::DatabaseSettings;
use crate::configuration::EmailClientSettings;
use crate::configuration::Settings;
use crate::helpers::get_server_address::get_server_address;
use crate::issue_delivery_worker::run_worker_until_stopped;
use crate::routes::get_providers_posts_route::get_providers_posts_route;
use crate::startup::Application;
use crate::startup::ApplicationBuildErrorEnum;
use crate::telemetry::get_subscriber;
use crate::telemetry::init_subscriber;
use secrecy::Secret;
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

#[actix_web::main] // or #[tokio::main]
pub async fn server_wrapper() -> Result<(), Box<ApplicationBuildErrorEnum>> {
    let subscriber = get_subscriber(
        "session_based_authentication".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);
    // let configuration = get_configuration().expect("Failed to read configuration.");
    let application = match Application::build().await {
        Ok(app) => app,
        Err(e) => return Err(e),
    };
    let application_task = tokio::spawn(application.run_until_stopped());
    let configuration = Settings {
        database: DatabaseSettings {
            host: CONFIG.postgres_ip.clone(),
            port: CONFIG.postgres_port,
            username: CONFIG.postgres_login.clone(),
            password: Secret::new(CONFIG.postgres_password.clone()),
            database_name: CONFIG.postgres_db.clone(),
            require_ssl: CONFIG.require_ssl,
        },
        application: ApplicationSettings {
            port: 8080,
            host: "127.0.0.1".to_string(),
            base_url: "http://127.0.0.1".to_string(),
            hmac_secret: Secret::new(
                "super-long-and-secret-random-key-needed-to-verify-message-integrity".to_string(),
            ),
        },
        email_client: EmailClientSettings {
            base_url: CONFIG.base_url.clone(),
            sender_email: "test@gmail.com".to_string(),
            authorization_token: Secret::new("my-secret-token".to_string()),
            timeout_milliseconds: 10000,
        },
        redis_uri: Secret::new("redis://127.0.0.1:6379".to_string()),
    };
    let worker_task = tokio::spawn(run_worker_until_stopped(configuration));
    tokio::select! {
        o = application_task => report_exit("API", o),
        o = worker_task => report_exit("Background worker", o),
    };
    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
