use crate::providers::provider_kind_enum::ProviderKind;

use crate::config_mods::config::CONFIG;

impl ProviderKind {
    pub fn get_mongo_collection_name(provider_kind: ProviderKind) -> String {
        format!(
            "{}{}",
            ProviderKind::get_string_name(provider_kind),
            CONFIG
                .mongo_params
                .providers_db_collection_handle_second_part
        )
    }
}