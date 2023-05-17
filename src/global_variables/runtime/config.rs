pub static CONFIG:  once_cell::sync::Lazy<tufa_common::repositories_types::tufa_server::config::config_struct::Config> =  once_cell::sync::Lazy::new(|| {
    tufa_common::repositories_types::tufa_server::config::config_struct::Config::try_from(
        tufa_common::repositories_types::tufa_server::config::config_struct::ConfigUnchecked::new()
        // .expect("cannot create config")
        // .wrap_config_checks()
        // .expect("wrap_config_checks error")
        .unwrap_or_else(|e| panic!("failed to ConfigUnchecked::new(), reason: {e:#?}"))
    ).unwrap_or_else(|e| panic!("failed to Config try_from ConfigUnchecked, reason: {e}"))
});
