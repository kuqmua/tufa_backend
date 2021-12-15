use std::collections::HashMap;

use diesel::prelude::*;

use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::get_providers_json_local_data_processed_error::GetProvidersJsonLocalDataProcessedError;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;

use crate::init_dbs::init_mongo::init_mongo;
use crate::init_dbs::init_mongo::MongoInitDbError;

use crate::init_dbs::init_postgres::init_postgres;
use crate::init_dbs::init_postgres::PostgresInitDbError;
//
#[derive(Debug)]
pub enum InitDbsError {
    GetProvidersJsonLocalData(HashMap<ProviderKind, GetProvidersJsonLocalDataProcessedError>),
    MongoClientOptionsParse(mongodb::error::Error),
    MongoClientWithOptions(mongodb::error::Error),
    MongoCollectionCountDocuments((ProviderKind, mongodb::error::Error)),
    MongoCollectionIsNotEmpty((ProviderKind, u64)),
    MongoCollectionInsertMany((ProviderKind, mongodb::error::Error)),
    PostgresLoadingProvidersLinkParts(diesel::result::Error),
    PostgresProvidersLinkPartsIsNotEmpty(Vec<QueryableLinkPart>),
    PostgresInsertPosts(diesel::result::Error),
    PostgresEstablishConnection(ConnectionError),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs() -> Result<(), InitDbsError> {
    let providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>;
    let providers_json_local_data_hashmap_clone: HashMap<ProviderKind, Vec<String>>;
    let (success_hashmap, errors_hashmap) = ProviderKind::get_providers_json_local_data_processed();
    if !errors_hashmap.is_empty() {
        return Err(InitDbsError::GetProvidersJsonLocalData(errors_hashmap));
    }
    // let mut f =  Vec::with_capacity(2);
    // if !CONFIG.mongo_enable_initialization {
    //     f.push(asyncinit_mongo(providers_json_local_data_hashmap));
    // }
    // if !CONFIG.postgres_enable_initialization {
    //     f.push(init_postgres(providers_json_local_data_hashmap_clone));
    // }
    providers_json_local_data_hashmap = success_hashmap.clone();
    providers_json_local_data_hashmap_clone = success_hashmap;
    

    let (mongo_insert_data_option_result, postgres_insert_data_option_result) = tokio::join!(
        //todo: remove option coz its just an vec of functions. enable logic can be checked before
        init_mongo(providers_json_local_data_hashmap),
        init_postgres(providers_json_local_data_hashmap_clone)
    );
    if let Err(err) = mongo_insert_data_option_result {
        match err {
            MongoInitDbError::ClientOptionsParse(e) => return Err(InitDbsError::MongoClientOptionsParse(e)),
            MongoInitDbError::ClientWithOptions(e) => return Err(InitDbsError::MongoClientWithOptions(e)),
            MongoInitDbError::CollectionCountDocuments((pk, e)) => return Err(InitDbsError::MongoCollectionCountDocuments((pk, e))),
            MongoInitDbError::CollectionIsNotEmpty((pk, documents_number)) => return Err(InitDbsError::MongoCollectionIsNotEmpty((pk, documents_number))),
            MongoInitDbError::CollectionInsertMany((pk, e)) => return Err(InitDbsError::MongoCollectionInsertMany((pk, e)))
        }
    }
    if let Err(err) = postgres_insert_data_option_result {
        match err {
            PostgresInitDbError::LoadingProvidersLinkParts(e) => {
                return Err(InitDbsError::PostgresLoadingProvidersLinkParts(e));
            }
            PostgresInitDbError::ProvidersLinkPartsIsNotEmpty(e_vec) => {
                return Err(InitDbsError::PostgresProvidersLinkPartsIsNotEmpty(e_vec));
            }
            PostgresInitDbError::InsertPosts(e) => {
                return Err(InitDbsError::PostgresInsertPosts(e));
            }
            PostgresInitDbError::EstablishConnection(e) => {
                return Err(InitDbsError::PostgresEstablishConnection(e));
            }
        }
    }
    Ok(())
}