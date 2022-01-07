use std::collections::HashMap;

use diesel::prelude::*;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;

use crate::init_dbs_logic::init_mongo::init_mongo;
// use crate::init_dbs_logic::init_mongo::InitMongoError;
use crate::init_dbs_logic::init_mongo::InitMongoErrorEnum;

use crate::init_dbs_logic::init_postgres::init_postgres;
use crate::init_dbs_logic::init_postgres::PostgresInitDbError;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind_impl::functions::get_local_data::ProvidersLocalDataError;
use crate::providers::providers_info::get_all_local_providers_data::get_all_local_providers_data;

use super::init_mongo::CountDocumentsError;

#[derive(Debug)]
pub enum InitDbsError {
    GetProvidersJsonLocalData(HashMap<ProviderKind, ProvidersLocalDataError>),
    MongoClient(mongodb::error::Error),
    MongoCountDocumentsError(HashMap<ProviderKind, CountDocumentsError>),
    MongoInsertManyError(HashMap<ProviderKind, mongodb::error::Error>),
    PostgresLoadingProvidersLinkParts(diesel::result::Error),
    PostgresProvidersLinkPartsIsNotEmpty(Vec<QueryableLinkPart>),
    PostgresInsertPosts(diesel::result::Error),
    PostgresEstablishConnection(ConnectionError),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs() -> Result<(), InitDbsError> {
    match get_all_local_providers_data().await {
        Err(errors_hashmap) => Err(InitDbsError::GetProvidersJsonLocalData(errors_hashmap)),
        Ok(success_hashmap) => {
            let providers_json_local_data_hashmap = success_hashmap.clone();
            let providers_json_local_data_hashmap_clone = success_hashmap;
            let (mongo_insert_data_option_result, postgres_insert_data_option_result) = tokio::join!(
                async {
                    if CONFIG.is_mongo_initialization_enabled {
                        return Some(init_mongo(providers_json_local_data_hashmap).await);
                    }
                    None
                },
                async {
                    if CONFIG.is_postgres_initialization_enabled {
                        return Some(init_postgres(providers_json_local_data_hashmap_clone).await);
                    }
                    None
                }
            );
            if let Some(Err(err)) = mongo_insert_data_option_result {
                let source = err.source;
                match *source {
                    InitMongoErrorEnum::Client(mongo_err) => {
                        return Err(InitDbsError::MongoClient(mongo_err));
                    }
                    InitMongoErrorEnum::CountDocumentsError(hashmap) => {
                        return Err(InitDbsError::MongoCountDocumentsError(hashmap));
                    }
                    InitMongoErrorEnum::InsertManyError(hashmap) => {
                        return Err(InitDbsError::MongoInsertManyError(hashmap));
                    }
                }
            }
            if let Some(Err(err)) = postgres_insert_data_option_result {
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
    }
}
