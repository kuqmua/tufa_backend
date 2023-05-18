pub async fn server_wrapper<'a>(
    config: &'a (
        impl tufa_common::traits::config_fields::GetPostgresIp
        + tufa_common::traits::config_fields::GetPostgresPort
        + tufa_common::traits::config_fields::GetPostgresLogin
        + tufa_common::traits::config_fields::GetPostgresPassword
        + tufa_common::traits::config_fields::GetPostgresDb
        + tufa_common::traits::config_fields::GetRequireSsl
        + tufa_common::traits::config_fields::GetServerPort
        + tufa_common::traits::config_fields::GetBaseUrl
        + tufa_common::traits::config_fields::GetHmacSecret
        + tufa_common::traits::config_fields::GetRedisIp
        + tufa_common::traits::config_fields::GetRedisPort

        + tufa_common::traits::config_fields::GetPostgresConnectionTimeout
        + tufa_common::traits::get_settings::GetSettings
    )
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed<'a>>> {
    let settings = config.get_settings();
    let application = match crate::startup::Application::build(
        settings.clone(),
        config
    ).await {
        Err(e) => return Err(Box::new(tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::ApplicationBuild {
            application_build: *e,
            code_occurence: tufa_common::code_occurence!(),
        })),
        Ok(app) => app,
    };
    let application_task = tokio::spawn(application.run_until_stopped()).await;
    //remove this coz too much spam
    match application_task {
        Err(e) => {
            return Err(Box::new(
                tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::TokioSpawn {
                    tokio_spawn: e,
                    code_occurence: tufa_common::code_occurence!(),
                },
            ));
        },
        Ok(result) => match result {
            Err(e) => {
                return Err(Box::new(tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::RunUntilStopped {
                    run_until_stopped: e,
                    code_occurence: tufa_common::code_occurence!(),
                }));
            },
            Ok(_) => (),
        },
    }
    let worker_task = tokio::spawn(crate::issue_delivery_worker::run_worker_until_stopped(settings));
    // tokio::select! {
    //     o = application_task => report_exit("API", o),
    //     o = worker_task => report_exit("Background worker", o),
    // };
    Ok(())
}

// fn report_exit(task_name: &str, outcome: Result<Result<(), impl std::fmt::Debug + std::fmt::Display>, tokio::task::JoinError>) {
//     match outcome {
//         Ok(Ok(())) => {
//             tracing::info!("{} has exited", task_name)
//         }
//         Ok(Err(e)) => {
//             tracing::error!(
//                 error.cause_chain = ?e,
//                 error.message = %e,
//                 "{} failed",
//                 task_name
//             )
//         }
//         Err(e) => {
//             tracing::error!(
//                 error.cause_chain = ?e,
//                 error.message = %e,
//                 "{}' task failed to complete",
//                 task_name
//             )
//         }
//     }
// }
