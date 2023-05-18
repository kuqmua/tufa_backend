pub fn entry<SelfGeneric>(
    config: &'static (
        impl tufa_common::traits::config_fields::GetTracingType

        + tufa_common::traits::config_fields::GetPostgresIp
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

        + tufa_common::traits::config_fields::GetServerPort
        + tufa_common::traits::config_fields::GetSourcePlaceType
        + tufa_common::traits::config_fields::GetTimezone

        + tufa_common::traits::config_fields::GetStartingCheckLink
        + tufa_common::traits::get_postgres_url::GetPostgresUrl<SelfGeneric>
        + tufa_common::traits::config_fields::GetPostgresConnectionTimeout

        + tufa_common::traits::get_postgres_database_settings::GetPostgresDatabaseSettings
        + tufa_common::traits::get_email_client::GetEmailClient

        + std::marker::Send 
        + std::marker::Sync
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
            if let Err(e) = tufa_common::repositories_types::tufa_server::telemetry::init_subscriber::init_subscriber(
                tufa_common::repositories_types::tufa_server::telemetry::get_subscriber::get_subscriber(
                env!("CARGO_PKG_VERSION"),
                config,
                std::io::stdout,
            ))
            {
                panic!("tufa_common::repositories_types::tufa_server::telemetry::init_subscriber::init_subscriber failed, error: {e:#?}")
            }
            else {
                //preparation logic must be enabled by default. service must check on existing database tables.
                if let Err(e) = runtime.block_on(crate::preparation::prepare_server::prepare_server(config)) {
                    use tufa_common::traits::error_logs_logic::error_log::ErrorLog;
                    e.error_log(config);
                    // let e_serialize_deserialize_version = e.into_serialize_deserialize_version();
                    // println!("{e_serialize_deserialize_version}");
                    // let e_json = serde_json::to_string(&e_serialize_deserialize_version).unwrap();
                    // println!("{e_json}");
                    // let e_deserialized: tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerErrorNamedWithSerializeDeserialize = serde_json::from_str(&e_json).unwrap();
                    // println!("{e_deserialized}");
                }
                if let Err(e) = runtime.block_on(crate::server_wrapper::server_wrapper(config)) {
                    eprintln!("server stopped");
                    use tufa_common::traits::error_logs_logic::error_log::ErrorLog;
                    e.error_log(config);
                }
            }
        }
    }
}
