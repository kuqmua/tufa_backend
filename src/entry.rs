pub fn entry() {
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
                crate::global_variables::runtime::config::CONFIG.log_type,
                tufa_common::repositories_types::tufa_server::telemetry::init_subscriber::init_subscriber(
                    tufa_common::repositories_types::tufa_server::telemetry::get_subscriber::get_subscriber(
                    crate::global_variables::hardcode::PROJECT_NAME.into(),
                    crate::global_variables::runtime::config::CONFIG.tracing_type.to_lower_snake_case(),
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
                    crate::global_variables::runtime::config::CONFIG.is_preparation_enabled,
                    runtime.block_on(crate::preparation::prepare_server::prepare_server(&{
                        use std::ops::Deref;
                        crate::global_variables::runtime::config::CONFIG.deref()
                    }))
                ) {
                    use tufa_common::traits::error_logs_logic::error_log::ErrorLog;
                    e.error_log(once_cell::sync::Lazy::force(
                        &crate::global_variables::runtime::config::CONFIG,
                    ));
                    // let e_serialize_deserialize_version = e.into_serialize_deserialize_version();
                    // println!("{e_serialize_deserialize_version}");
                    // let e_json = serde_json::to_string(&e_serialize_deserialize_version).unwrap();
                    // println!("{e_json}");
                    // let e_deserialized: tufa_common::repositories_types::tufa_server::preparation::prepare_server::PrepareServerErrorNamedWithSerializeDeserialize = serde_json::from_str(&e_json).unwrap();
                    // println!("{e_deserialized}");
                }
                // if let Err(e) = crate::server_wrapper::server_wrapper() {
                // }
            }
        }
    }
}
