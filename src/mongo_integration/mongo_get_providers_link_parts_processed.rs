use std::collections::HashMap;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::mongo_integration::mongo_get_providers_link_parts_unprocessed::mongo_get_providers_link_parts_unprocessed;

#[derive(Debug)]
pub enum MongoGetProvidersLinkPartsProcessedResult {
    MongoAgregationOrCursorTryNext(HashMap<ProviderKind, mongodb::error::Error>),
    MongoConnection(mongodb::error::Error),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_get_providers_link_parts_processed(
) -> Result<HashMap<ProviderKind, Vec<String>>, MongoGetProvidersLinkPartsProcessedResult> {
    match mongo_get_providers_link_parts_unprocessed().await {
        Err(e) => Err(MongoGetProvidersLinkPartsProcessedResult::MongoConnection(
            e,
        )),
        Ok(unprocessed_hashmap) => {
            let mut success_hashmap: HashMap<ProviderKind, Vec<String>> =
                HashMap::with_capacity(unprocessed_hashmap.len());
            let mut error_hashmap: HashMap<ProviderKind, mongodb::error::Error> =
                HashMap::with_capacity(unprocessed_hashmap.len());
            for (provider_kind, result_vec) in unprocessed_hashmap {
                match result_vec {
                    Ok(vec) => {
                        success_hashmap.insert(provider_kind, vec);
                    }
                    Err(e) => {
                        error_hashmap.insert(provider_kind, e);
                    }
                }
            }
            if !error_hashmap.is_empty() {
                return Err(
                    MongoGetProvidersLinkPartsProcessedResult::MongoAgregationOrCursorTryNext(
                        error_hashmap,
                    ),
                );
            }
            Ok(success_hashmap)
        }
    }
}
