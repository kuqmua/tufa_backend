pub fn entry<SelfGeneric>(
    config: &(
        impl tufa_common::traits::fields::GetLogType
        + tufa_common::traits::fields::GetTracingType
        + tufa_common::traits::fields::GetIsPreparationEnabled

        + tufa_common::traits::fields::GetPostgresIp
        + tufa_common::traits::fields::GetPostgresPort
        + tufa_common::traits::fields::GetPostgresLogin
        + tufa_common::traits::fields::GetPostgresPassword
        + tufa_common::traits::fields::GetPostgresDb
        + tufa_common::traits::fields::GetRequireSsl
        + tufa_common::traits::fields::GetServerPort
        + tufa_common::traits::fields::GetServerIp
        + tufa_common::traits::fields::GetBaseUrl
        + tufa_common::traits::fields::GetHmacSecret
        + tufa_common::traits::fields::GetRedisIp
        + tufa_common::traits::fields::GetRedisPort

        + tufa_common::traits::get_color::ErrorColorBold
        + tufa_common::traits::fields::GetServerPort
        + tufa_common::traits::fields::GetSourcePlaceType
        + tufa_common::traits::fields::GetTimezone

        + tufa_common::traits::fields::GetStartingCheckLink
        + tufa_common::traits::get_postgres_url::GetPostgresUrl<SelfGeneric>
        + tufa_common::traits::fields::GetPostgresConnectionTimeout
        + tufa_common::traits::fields::GetMongoProvidersLogsDbName

        + tufa_common::traits::fields::GetIsDbsInitializationEnabled
        + tufa_common::traits::fields::GetIsMongoInitializationEnabled
        + tufa_common::traits::fields::GetIsPostgresInitializationEnabled
    )
) {
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(e) => {
            panic!("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {e:#?}")
        }
        Ok(runtime) => {
            if let (
                tufa_common::config_mods::log_type::LogType::Tracing, 
                Err(e)
            ) = (
                config.get_log_type(),
                tufa_common::repositories_types::tufa_server::telemetry::init_subscriber::init_subscriber(
                    tufa_common::repositories_types::tufa_server::telemetry::get_subscriber::get_subscriber(
                    env!("CARGO_PKG_VERSION"),
                    config.get_tracing_type().to_lower_snake_case(),
                    std::io::stdout,
                ))
            ) {
                panic!("tufa_common::repositories_types::tufa_server::telemetry::init_subscriber::init_subscriber failed, error: {e:#?}")
            }
            else {
                if let (
                    true, 
                    Err(e)
                ) = (
                    config.get_is_preparation_enabled(),
                    runtime.block_on(crate::preparation::prepare_server::prepare_server(config))
                ) {
                    use tufa_common::traits::error_logs_logic::error_log::ErrorLog;
                    e.error_log(config);
                    // let e_serialize_deserialize_version = e.into_serialize_deserialize_version();
                    // println!("{e_serialize_deserialize_version}");
                    // let e_json = serde_json::to_string(&e_serialize_deserialize_version).unwrap();
                    // println!("{e_json}");
                    // let e_deserialized: tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerErrorNamedWithSerializeDeserialize = serde_json::from_str(&e_json).unwrap();
                    // println!("{e_deserialized}");
                }
                // if let Err(e) = crate::server_wrapper::server_wrapper(config) {
                // }
            }
        }
    }
}
