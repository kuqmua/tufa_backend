use crate::providers::provider_kind_enum::ProviderKind;

use crate::config_mods::lazy_static_config::CONFIG;

impl ProviderKind {
    pub fn get_init_local_data_file_path(provider_kind: ProviderKind) -> String {
        format!(
            "{}{}{}{}",
            CONFIG.mongo_params.path_to_provider_link_parts_folder,
            ProviderKind::get_string_name(provider_kind),
            CONFIG
                .mongo_params
                .providers_db_collection_handle_second_part,
            CONFIG.mongo_params.log_file_extension
        )
    }
}