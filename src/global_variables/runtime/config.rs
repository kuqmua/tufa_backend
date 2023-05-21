pub static CONFIG:  once_cell::sync::Lazy<tufa_common::repositories_types::tufa_server::config::config_struct::Config> =  once_cell::sync::Lazy::new(|| {
    tufa_common::repositories_types::tufa_server::config::config_struct::Config::try_from_config_unchecked(
        tufa_common::repositories_types::tufa_server::config::config_struct::ConfigUnchecked::new()
        .unwrap_or_else(|e| panic!("failed to ConfigUnchecked::new(), reason: {e:#?}"))
    ).unwrap_or_else(|e| panic!("failed to Config try_from ConfigUnchecked, reason: {e}"))
});

//Arc<RwLock<Store>>