pub fn entry<'a>(
    config: &'static tufa_common::repositories_types::tufa_server::config::config_struct::Config,
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
            tufa_common::dev::dev();
            crate::dev::dev();
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
                println!("checking net availability...");
                if let Err(e) = runtime.block_on(tufa_common::server::net::net_check_availability::net_check_availability(config)) {
                    use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                    e.error_log(&config);
                    // let e_serialize_deserialize_version = e.into_serialize_deserialize_version();
                    // println!("{e_serialize_deserialize_version}");
                    // let e_json = serde_json::to_string(&e_serialize_deserialize_version).unwrap();
                    // println!("{e_json}");
                    // let e_deserialized: tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerErrorNamedWithSerializeDeserialize = serde_json::from_str(&e_json).unwrap();
                    // println!("{e_deserialized}");
                }
                if let Err(e) = runtime.block_on(crate::server_wrapper::server_wrapper(&config)) {
                    eprintln!("server stopped");
                    use tufa_common::common::error_logs_logic::error_log::ErrorLog;
                    e.error_log(&config);
                }
            }
        }
    }
}
