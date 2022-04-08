use crate::helpers::get_server_address::get_server_address;
use crate::issue_delivery_worker::run_worker_until_stopped;
use crate::routes::get_providers_posts_route::get_providers_posts_route;
use crate::startup::Application;
use crate::startup::ApplicationBuildErrorEnum;
use crate::telemetry::get_subscriber;
use crate::telemetry::init_subscriber;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

#[actix_web::main] // or #[tokio::main]
pub async fn server_wrapper() {
    // HttpServer::new(|| {
    //     App::new()
    //         // .route("/", web::get().to(|| async { "Hello World!" }))
    //         .route(
    //             "/get_providers_posts",
    //             web::post().to(get_providers_posts_route),
    //         )
    // })
    // .bind(get_server_address())?
    // .run()
    // .await
    let subscriber = get_subscriber(
        "session_based_authentication".into(),
        "info".into(),
        std::io::stdout,
    );
    // subscriber
    init_subscriber(subscriber);
    // let configuration = get_configuration().expect("Failed to read configuration.");
    let application = match Application::build().await {
        Ok(app) => app,
        Err(e) => panic!("error"),
    };
    let application_task = tokio::spawn(application.run_until_stopped());
    // let worker_task = tokio::spawn(run_worker_until_stopped(configuration));
    // tokio::select! {
    //     o = application_task => report_exit("API", o),
    //     o = worker_task => report_exit("Background worker", o),
    // };
    // return None;
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
