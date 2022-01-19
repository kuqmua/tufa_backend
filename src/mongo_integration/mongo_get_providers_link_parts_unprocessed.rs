use std::collections::HashMap;
use std::fmt;

use mongodb::{bson::Document, options::ClientOptions, Client};

use crate::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector;
use crate::{
    config_mods::lazy_static_config::CONFIG, traits::provider_kind_trait::ProviderKindTrait,
};

use crate::{
    mongo_integration::mongo_get_db_url::mongo_get_db_url,
    providers::provider_kind_enum::ProviderKind,
};

use futures::future::join_all;

use super::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorError;

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct MongoGetProvidersLinkPartsUnprocessedError {
    pub source: Box<MongoGetProvidersLinkPartsUnprocessedErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum MongoGetProvidersLinkPartsUnprocessedErrorEnum {
    ClientOptionsParse(ClientOptionsParseError),
    ClientWithOptions(ClientWithOptionsError),
    ListCollectionNames(ListCollectionNamesError),
    NoSuchCollections(HashMap<ProviderKind, String>),
    GetDocuments(HashMap<ProviderKind, MongoGetDocumentsAsStringVectorError>),
}

#[derive(Debug)]
pub struct ClientOptionsParseError {
    pub source: mongodb::error::Error,
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
}

#[derive(Debug)]
pub struct ListCollectionNamesError {
    pub source: mongodb::error::Error,
}

#[deny(clippy::indexing_slicing)] //, clippy::unwrap_used
pub async fn mongo_get_providers_link_parts_unprocessed(
) -> Result<HashMap<ProviderKind, Vec<String>>, MongoGetProvidersLinkPartsUnprocessedError> {
    //todo: write without arc - removing unwrap
    let client_options: ClientOptions;
    match ClientOptions::parse(mongo_get_db_url()).await {
        Err(e) => {
            return Err(MongoGetProvidersLinkPartsUnprocessedError {
                source: Box::new(
                    MongoGetProvidersLinkPartsUnprocessedErrorEnum::ClientOptionsParse(
                        ClientOptionsParseError { source: e },
                    ),
                ),
            })
        }
        Ok(cl) => client_options = cl,
    }
    let client: Client;
    match Client::with_options(client_options) {
        Err(e) => {
            return Err(MongoGetProvidersLinkPartsUnprocessedError {
                source: Box::new(
                    MongoGetProvidersLinkPartsUnprocessedErrorEnum::ClientWithOptions(
                        ClientWithOptionsError { source: e },
                    ),
                ),
            })
        }
        Ok(c) => client = c,
    }
    let db = client.database(&CONFIG.mongo_providers_link_parts_db_name);
    let vec_collection_names: Vec<String>;
    //todo ProviderKind::get_enabled_providers_vec as filter
    match db.list_collection_names(None).await {
        Err(e) => {
            return Err(MongoGetProvidersLinkPartsUnprocessedError {
                source: Box::new(
                    MongoGetProvidersLinkPartsUnprocessedErrorEnum::ListCollectionNames(
                        ListCollectionNamesError { source: e },
                    ),
                ),
            })
        }
        Ok(vcn) => vec_collection_names = vcn,
    }
    let no_collection_error_hashmap = ProviderKind::get_enabled_providers_vec()
        .into_iter()
        .filter_map(|pk| {
            let collection_name = pk.get_mongo_log_collection_name();
            if !vec_collection_names.contains(&collection_name) {
                return Some((pk, collection_name));
            }
            None
        })
        .collect::<HashMap<ProviderKind, String>>();
    if !no_collection_error_hashmap.is_empty() {
        return Err(MongoGetProvidersLinkPartsUnprocessedError {
            source: Box::new(
                MongoGetProvidersLinkPartsUnprocessedErrorEnum::NoSuchCollections(
                    no_collection_error_hashmap,
                ),
            ),
        });
    }
    let enabled_providers_vec = ProviderKind::get_enabled_providers_vec();
    let result_get_documents_hashmap = join_all(enabled_providers_vec.iter().map(|pk| async {
        let pk_cloned = pk.clone();
        (
            pk_cloned,
            mongo_get_documents_as_string_vector(
                db.collection::<Document>(&pk.get_mongo_log_collection_name()),
                &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle,
                ProviderKind::get_mongo_provider_link_parts_aggregation(&pk_cloned),
            )
            .await,
        )
    }))
    .await;
    let get_documents_error_hashmap: HashMap<ProviderKind, MongoGetDocumentsAsStringVectorError> =
        HashMap::new();
    if !get_documents_error_hashmap.is_empty() {
        return Err(MongoGetProvidersLinkPartsUnprocessedError {
            source: Box::new(
                MongoGetProvidersLinkPartsUnprocessedErrorEnum::GetDocuments(
                    get_documents_error_hashmap,
                ),
            ),
        });
    }

    Ok(result_get_documents_hashmap
        .into_iter()
        .filter_map(|(pk, result)| {
            if let Ok(vec) = result {
                return Some((pk, vec));
            }
            None
        })
        .collect())
}
