use crate::providers::providers_info::get_providers_link_parts_from_mongo::get_providers_link_parts_from_mongo;

use std::collections::HashMap;

use crate::helpers::resource::Resource;

use crate::providers::provider_kind_enum::ProviderKind;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_as_hashmap(
    resource: &Resource,
) -> HashMap<ProviderKind, Vec<String>> {
    let vec_of_link_parts_hashmap: HashMap<ProviderKind, Vec<String>>;
    match resource {
        Resource::Local => {
            vec_of_link_parts_hashmap = ProviderKind::get_providers_json_local_data();
        }
        Resource::Mongodb => {
            vec_of_link_parts_hashmap = get_providers_link_parts_from_mongo().await;
        }
        Resource::PostgreSql => {
            todo!()
        }
    }
    vec_of_link_parts_hashmap
}
