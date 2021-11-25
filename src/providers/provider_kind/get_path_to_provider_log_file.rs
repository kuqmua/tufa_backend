use crate::providers::provider_kind_enum::ProviderKind;

use crate::config_mods::lazy_static_config::CONFIG;

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_path_to_provider_log_file(provider_kind: ProviderKind) -> String {
        format!(
            "logs/{}/{:?}/{}",
            &CONFIG.params.warning_logs_directory_name,
            provider_kind,
            &CONFIG
                .params
                .unhandled_success_handled_success_are_there_items_initialized_posts_dir
        )
    }
}