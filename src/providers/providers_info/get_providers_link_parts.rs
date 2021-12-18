use std::collections::HashMap;

use crate::helpers::resource::Resource;

use crate::mongo_integration::mongo_get_providers_link_parts_processed::mongo_get_providers_link_parts_processed;
use crate::mongo_integration::mongo_get_providers_link_parts_processed::MongoGetProvidersLinkPartsProcessedResult;
use crate::providers::provider_kind_enum::ProviderKind;

use crate::providers::provider_kind_impl::functions::get_local_data::ProvidersLocalDataError;
use crate::providers::providers_info::get_all_local_providers_data::get_all_local_providers_data;

#[derive(Debug)]
pub enum GetLinkPartsError {
    Local(HashMap<ProviderKind, ProvidersLocalDataError>),
    Mongodb(MongoGetProvidersLinkPartsProcessedResult),
    PostgreSql, //todo
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_as_hashmap(
    resource: &Resource,
) -> Result<HashMap<ProviderKind, Vec<String>>, GetLinkPartsError> {
    match resource {
        Resource::Local => match get_all_local_providers_data().await {
            Err(error_hashmap) => Err(GetLinkPartsError::Local(error_hashmap)),
            Ok(success_hashmap) => Ok(success_hashmap),
        },
        Resource::Mongodb => match mongo_get_providers_link_parts_processed().await {
            Err(e) => Err(GetLinkPartsError::Mongodb(e)),
            Ok(success_hashmap) => Ok(success_hashmap),
        },
        Resource::PostgreSql => {
            todo!()
        }
    }
}
