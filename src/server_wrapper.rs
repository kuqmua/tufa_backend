pub async fn server_wrapper<'a>(
    config: &'static (
        impl tufa_common::traits::config_fields::GetServerPort
        + tufa_common::traits::config_fields::GetBaseUrl
        + tufa_common::traits::config_fields::GetHmacSecret
        + tufa_common::traits::config_fields::GetRedisIp
        + tufa_common::traits::config_fields::GetRedisPort
        + tufa_common::traits::config_fields::GetSourcePlaceType
        + tufa_common::traits::config_fields::GetTimezone
        + tufa_common::traits::config_fields::GetAccessControlMaxAge
        + tufa_common::traits::config_fields::GetAccessControlAllowOrigin
        + tufa_common::traits::config_fields::GetPostgresPool
        + tufa_common::traits::get_email_client::GetEmailClient
        + tufa_common::traits::get_server_address::GetServerAddress
        + tufa_common::traits::try_create_tcp_listener::TryCreateTcpListener<'a>

        + std::marker::Send 
        + std::marker::Sync
    )
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed<'a>>> {
    let actix_web_dev_server = match crate::try_build_actix_web_dev_server::try_build_actix_web_dev_server(config).await {
        Err(e) => return Err(Box::new(tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::ApplicationBuild {
            application_build: *e,
            code_occurence: tufa_common::code_occurence!(),
        })),
        Ok(app) => app,
    };
    let application_task = tokio::spawn(async move {
        match 
            actix_web_dev_server
            .await 
        {
            Err(e) => Err(tufa_common::repositories_types::tufa_server::server_wrapper::RunUntilStoppedErrorNamed::RunUntilStopped {
                run_until_stopped: e,
                code_occurence: tufa_common::code_occurence!(),
            }),
            Ok(_) => Ok(()),
        }
    });
    // let worker_task = tokio::spawn(crate::issue_delivery_worker::run_worker_until_stopped(config));
    tokio::select! {
        task = application_task => match task {
            Ok(Ok(())) => (),
            Ok(Err(e)) => {
                return Err(Box::new(tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::RunUntilStopped {
                    run_until_stopped: e,
                    code_occurence: tufa_common::code_occurence!(),
                }));
            },
            Err(e) => {
                return Err(Box::new(
                    tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::TokioSpawn {
                        tokio_spawn: e,
                        code_occurence: tufa_common::code_occurence!(),
                    },
                ));
            },
        },
        // task = worker_task => match task {
        //     Ok(_) => println!("2"),
        //     Err(_) => println!("3"),
        // },//report_exit("Background worker", o)
    };
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
