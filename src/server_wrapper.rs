pub async fn server_wrapper<'a>(
    config: &'static tufa_common::repositories_types::tufa_server::config::config_struct::Config,
) -> Result<
    (),
    Box<tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed<'a>>,
> {
    //todo check postgres version with sql query
    // let tcp_listener = {
    //     use tufa_common::common::config::try_create_tcp_listener::TryCreateTcpListener;
    //     config.try_create_tcp_listener()
    // }.or_else(|e| Err(Box::new(
    //     tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::TcpListenerBind {
    //         tcp_listener_bind: *e,
    //         code_occurence: tufa_common::code_occurence!(),
    //     }
    // )))?;
    // let tcp_listener = match {
    //     use tufa_common::common::config::try_create_tcp_listener::TryCreateTcpListener;
    //     config.try_create_tcp_listener()
    // } {
    //     Ok(tcp_listener) => tcp_listener,
    //     Err(e) => {
    //         return Err(Box::new(
    //             tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::TcpListenerBind {
    //                 tcp_listener_bind: *e,
    //                 code_occurence: tufa_common::code_occurence!(),
    //             }
    //         ))
    //     },
    // };
    println!("trying to create postgres pool...");
    let postgres_pool = match {
        use tufa_common::common::config::try_get_postgres_pool::TryGetPostgresPool;
        config.try_get_postgres_pool().await
    } {
        Ok(postgres_pool) => postgres_pool,
        Err(e) => {
            return Err(Box::new(
                tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::TryGetPostgresPool {
                    try_get_postgres_pool: e,
                    code_occurence: tufa_common::code_occurence!(),
                }
            ))
        },
    };
    // println!("trying to create redis session storage...");
    // let redis_session_storage = match {
    //     use tufa_common::common::config::try_get_redis_session_storage::TryGetRedisSessionStorage;
    //     config.try_get_redis_session_storage().await
    // } {
    //     Ok(redis_session_storage) => redis_session_storage,
    //     Err(e) => {
    //         return Err(Box::new(
    //             tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::TryGetRedisSessionStorage {
    //                 try_get_redis_session_storage: e,
    //                 code_occurence: tufa_common::code_occurence!(),
    //             }
    //         ))
    //     },
    // };
    println!("trying to build server...");
    match crate::try_build_actix_web_dev_server::try_build_actix_web_dev_server(
        // tcp_listener,
        postgres_pool,
        // redis_session_storage,
        config
    ).await {
        Err(e) => return Err(Box::new(tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::ApplicationBuild {
            application_build: *e,
            code_occurence: tufa_common::code_occurence!(),
        })),
        Ok(_) => (),
    }
    println!("server running!");
    // let application_task = tokio::spawn(async move {
    //     match
    //         actix_web_dev_server
    //         .await
    //     {
    //         Err(e) => Err(tufa_common::repositories_types::tufa_server::server_wrapper::RunUntilStoppedErrorNamed::RunUntilStopped {
    //             run_until_stopped: e,
    //             code_occurence: tufa_common::code_occurence!(),
    //         }),
    //         Ok(_) => Ok(()),
    //     }
    // });
    // let worker_task = tokio::spawn(crate::issue_delivery_worker::run_worker_until_stopped(config));
    // tokio::select! {
    //     task = application_task => match task {
    //         Ok(Ok(())) => (),
    //         Ok(Err(e)) => {
    //             return Err(Box::new(tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::RunUntilStopped {
    //                 run_until_stopped: e,
    //                 code_occurence: tufa_common::code_occurence!(),
    //             }));
    //         },
    //         Err(e) => {
    //             return Err(Box::new(
    //                 tufa_common::repositories_types::tufa_server::server_wrapper::ServerWrapperErrorNamed::TokioSpawn {
    //                     tokio_spawn: e,
    //                     code_occurence: tufa_common::code_occurence!(),
    //                 },
    //             ));
    //         },
    //     },
    //     // task = worker_task => match task {
    //     //     Ok(_) => println!("2"),
    //     //     Err(_) => println!("3"),
    //     // },//report_exit("Background worker", o)
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
