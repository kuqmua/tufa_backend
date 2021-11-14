use std::collections::HashMap;

use crate::helpers::resource::Resource;

use crate::providers::provider_kind_enum::ProviderKind;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_as_hashmap(
    resource: &Resource,
) -> HashMap<ProviderKind, Vec<String>> {
    //todo: return different type as errors or success enum
    match resource {
        Resource::Local => ProviderKind::get_providers_json_local_data_processed(
            ProviderKind::get_providers_json_local_data_unprocessed(),
        ),
        Resource::Mongodb => ProviderKind::mongo_get_providers_link_parts_processed(
            ProviderKind::mongo_get_providers_link_parts_unprocessed(
                ProviderKind::get_enabled_providers_vec(),
            )
            .await,
        ),
        Resource::PostgreSql => {
            todo!()
        }
    }
}
