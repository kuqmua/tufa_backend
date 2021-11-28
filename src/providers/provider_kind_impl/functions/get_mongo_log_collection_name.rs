use crate::providers::provider_kind_enum::ProviderKind;

use crate::config_mods::lazy_static_config::CONFIG;

impl ProviderKind {
    pub fn get_mongo_log_collection_name(provider_kind: ProviderKind) -> String {
        format!(
            "{}{}",
            provider_kind,
            CONFIG
                .mongo_params
                .providers_db_collection_handle_second_part //todo rename it into db log collection
        )
    }
}
