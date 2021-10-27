use crate::providers::providers_info::get_providers_json_local_data::get_providers_json_local_data;
use crate::providers::providers_info::get_providers_link_parts_from_mongo::get_providers_link_parts_from_mongo;

use std::collections::HashMap;

use crate::helpers::resource::Resource;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_as_hashmap(
    resource: &Resource,
) -> HashMap<&'static str, Vec<String>> {
    let vec_of_link_parts_hashmap: HashMap<&'static str, Vec<String>>;
    match resource {
        Resource::Local => {
            vec_of_link_parts_hashmap = get_providers_json_local_data();
        }
        Resource::Mongodb {
            providers_string_into_enum_hashmap,
        } => {
            vec_of_link_parts_hashmap = get_providers_link_parts_from_mongo(
                providers_string_into_enum_hashmap.clone(),
            )
            .await;
        }
        Resource::PostgreSql => {
            todo!()
        }
    }
    vec_of_link_parts_hashmap
}
