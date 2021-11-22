use std::collections::HashMap;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::providers::providers_info::get_providers_link_parts::get_providers_link_parts_as_hashmap;

use crate::helpers::resource::Resource;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_wrapper(resource: &Resource ) -> HashMap<ProviderKind, Vec<String>> {
    //todo: write Resource logic
    match resource {
        Resource::Local => get_providers_link_parts_as_hashmap(resource).await,
        Resource::Mongodb => get_providers_link_parts_as_hashmap(resource).await,
        Resource::PostgreSql => todo!(),//get_providers_link_parts_wrapper still exists coz there is no postgres implementation
    }
}
