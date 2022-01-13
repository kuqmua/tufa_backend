use std::collections::HashMap;


use crate::config_mods::lazy_static_config::CONFIG;

use crate::init_dbs_logic::init_mongo::init_mongo;
use crate::init_dbs_logic::init_mongo::InitMongoErrorEnum;

use crate::init_dbs_logic::init_postgres::init_postgres;
use crate::init_dbs_logic::init_postgres::PostgresInitErrorEnum;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind_impl::functions::get_local_data::ProvidersLocalDataError;
use crate::providers::providers_info::get_all_local_providers_data::get_all_local_providers_data;

use super::init_mongo::CollectionCountDocumentsOrIsNotEmpty;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsError;

use crate::postgres_integration::postgres_check_provider_links_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyError;

#[derive(Debug)]
pub enum InitDbsError {
    GetProvidersJsonLocalData(HashMap<ProviderKind, ProvidersLocalDataError>),
    MongoClient(mongodb::error::Error),
    MongoCollectionCountDocumentsOrIsNotEmpty(
        HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>,
    ),
    MongoInsertManyError(HashMap<ProviderKind, mongodb::error::Error>),
    // PostgresLoadingProvidersLinkParts(diesel::result::Error),
    PostgresCheckProvidersLinkPartsTablesEmptyError(PostgresCheckProvidersLinkPartsTablesEmptyError),
    PostgresCreateTableQueries(PostgresCreateProvidersDbsError),
    PostgresInsertQueries(HashMap<ProviderKind, sqlx::Error>),
    PostgresEstablishConnection(sqlx::Error),
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
                match *err.source {
                    InitMongoErrorEnum::Client(mongo_err) => {
                        return Err(InitDbsError::MongoClient(mongo_err));
                    }
                    InitMongoErrorEnum::CollectionCountDocumentsOrIsNotEmpty(hashmap) => {
                        return Err(InitDbsError::MongoCollectionCountDocumentsOrIsNotEmpty(
                            hashmap,
                        ));
                    }
                    InitMongoErrorEnum::InsertManyError(hashmap) => {
                        return Err(InitDbsError::MongoInsertManyError(hashmap));
                    }
                }
            }
            if let Some(Err(err)) = postgres_insert_data_option_result {
                match *err.source {
                    // PostgresInitErrorEnum::LoadingProvidersLinkParts(e) => {
                    //     return Err(InitDbsError::PostgresLoadingProvidersLinkParts(e));
                    // }
                    PostgresInitErrorEnum::CheckProviderLinksTablesAreEmpty(e) => {
                        return Err(InitDbsError::PostgresCheckProvidersLinkPartsTablesEmptyError(e));
                    }
                    PostgresInitErrorEnum::CreateTableQueries(e) => {
                        return Err(InitDbsError::PostgresCreateTableQueries(e));
                    }
                    PostgresInitErrorEnum::InsertQueries(e) => {
                        return Err(InitDbsError::PostgresInsertQueries(e));
                    }
                    PostgresInitErrorEnum::EstablishConnection(e) => {
                        return Err(InitDbsError::PostgresEstablishConnection(e));
                    }
                }
            }
            Ok(())
        }
    }
}
