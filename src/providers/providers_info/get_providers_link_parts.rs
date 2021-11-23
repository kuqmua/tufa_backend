use std::collections::HashMap;

use crate::helpers::resource::Resource;

use crate::providers::provider_kind_enum::{GetProvidersJsonLocalDataProcessedError, MongoGetProvidersLinkPartsProcessedResult, ProviderKind};

#[derive(Debug)]
pub enum LocalResourceErrorStatus {
    Local(HashMap<ProviderKind, GetProvidersJsonLocalDataProcessedError>),
    Mongodb(MongoGetProvidersLinkPartsProcessedResult),
    PostgreSql, //todo
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_as_hashmap(
    resource: &Resource,
) -> (HashMap<ProviderKind, Vec<String>>, LocalResourceErrorStatus) {
    //todo: return different type as errors or success enum
    //todo: write here converison to common return type
    match resource {
        Resource::Local => {
            let (success_hashmap, errors_hashmap) =
                ProviderKind::get_providers_json_local_data_processed();
            (success_hashmap, LocalResourceErrorStatus::Local(errors_hashmap))
        }
        // HashMap<ProviderKind, Result<Result<Vec<String>, serde_json::Error>, std::io::Error>>
        Resource::Mongodb => {
            let (success_hashmap, mongo_get_providers_link_part_processed_result) = ProviderKind::mongo_get_providers_link_parts_processed().await;
            (success_hashmap, LocalResourceErrorStatus::Mongodb(mongo_get_providers_link_part_processed_result))
        },
        Resource::PostgreSql => {
            todo!()
        }
    }
}
