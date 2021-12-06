use std::collections::HashMap;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::mongo_integration::mongo_get_providers_link_parts_unprocessed::mongo_get_providers_link_parts_unprocessed;

#[derive(Debug)]
pub enum MongoGetProvidersLinkPartsProcessedResult {
    MongoDocuments(HashMap<ProviderKind, mongodb::error::Error>),
    MongoConnection(mongodb::error::Error),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn mongo_get_providers_link_parts_processed() -> (
    HashMap<ProviderKind, Vec<String>>,
    MongoGetProvidersLinkPartsProcessedResult,
) {
    //HashMap<ProviderKind, Vec<String>>
    match mongo_get_providers_link_parts_unprocessed().await {
        Ok(unprocessed_hashmap) => {
            let mut first_return_handle: HashMap<ProviderKind, Vec<String>> =
                HashMap::with_capacity(unprocessed_hashmap.len());
            let mut second_return_handle: HashMap<ProviderKind, mongodb::error::Error> =
                HashMap::with_capacity(unprocessed_hashmap.len());
            for (provider_kind, result_vec) in unprocessed_hashmap {
                match result_vec {
                    Ok(vec) => {
                        first_return_handle.insert(provider_kind, vec);
                    }
                    Err(e) => {
                        print_colorful_message(
                                    Some(&provider_kind),
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("(todo!)ProviderKind::mongo_get_providers_link_parts_processed ({:#?}), error: {:#?}", provider_kind, e),
                                );
                        second_return_handle.insert(provider_kind, e);
                    }
                }
            }
            (
                first_return_handle,
                MongoGetProvidersLinkPartsProcessedResult::MongoDocuments(second_return_handle),
            )
        }
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "(todo!)ProviderKind::mongo_get_providers_link_parts_processed error: {:#?}",
                    e
                ),
            );
            (
                HashMap::new(),
                MongoGetProvidersLinkPartsProcessedResult::MongoConnection(e),
            )
        }
    }
}
