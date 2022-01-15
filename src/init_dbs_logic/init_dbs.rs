use std::collections::HashMap;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::init_dbs_logic::init_mongo::init_mongo;
use crate::init_dbs_logic::init_mongo::InitMongoErrorEnum;
use crate::init_dbs_logic::init_postgres::init_postgres;
use crate::init_dbs_logic::init_postgres::PostgresInitErrorEnum;

use crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind_impl::functions::get_local_data::ProvidersLocalDataError;
use crate::providers::providers_info::get_all_local_providers_data::get_all_local_providers_data;

use super::init_mongo::CollectionCountDocumentsOrIsNotEmpty;

use crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyError;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsError;
use crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesError;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesError;

#[derive(Debug)]
pub enum InitDbsError {
    GetProvidersJsonLocalData(HashMap<ProviderKind, ProvidersLocalDataError>),
    NoProvidersInsideLocalProvidersData,
    MongoClient(mongodb::error::Error),
    MongoCollectionCountDocumentsOrIsNotEmpty(
        HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>,
    ),
    MongoInsertManyError(HashMap<ProviderKind, mongodb::error::Error>),
    PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLength(
        PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError,
    ),
    PostgresDeleteAllFromProvidersTables(PostgresDeleteAllFromProvidersTablesError),
    PostgresCheckProvidersLinkPartsTablesEmptyError(
        PostgresCheckProvidersLinkPartsTablesEmptyError,
    ),
    PostgresCreateTableQueries(PostgresCreateProvidersDbsError),
    PostgresInsertLinkPartsIntoProvidersTables(PostgresInsertLinkPartsIntoProvidersTablesError),
    PostgresEstablishConnection(sqlx::Error),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs() -> Result<(), InitDbsError> {
    match get_all_local_providers_data().await {
        Err(errors_hashmap) => Err(InitDbsError::GetProvidersJsonLocalData(errors_hashmap)),
        Ok(providers_json_local_data_hashmap) => {
            if providers_json_local_data_hashmap.is_empty() {
                return Err(InitDbsError::NoProvidersInsideLocalProvidersData);
            }
            let providers_json_local_data_hashmap_clone = providers_json_local_data_hashmap.clone();
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
                    //
                    PostgresInitErrorEnum::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength(e) => {
                        return Err(InitDbsError::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLength(e));
                    }
                    //
                    PostgresInitErrorEnum::DeleteAllFromProvidersTables(e) => {
                        return Err(InitDbsError::PostgresDeleteAllFromProvidersTables(e));
                    }
                    PostgresInitErrorEnum::CheckProviderLinksTablesAreEmpty(e) => {
                        return Err(
                            InitDbsError::PostgresCheckProvidersLinkPartsTablesEmptyError(e),
                        );
                    }
                    PostgresInitErrorEnum::CreateTableQueries(e) => {
                        return Err(InitDbsError::PostgresCreateTableQueries(e));
                    }
                    PostgresInitErrorEnum::InsertLinkPartsIntoProvidersTables(e) => {
                        return Err(InitDbsError::PostgresInsertLinkPartsIntoProvidersTables(e));
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
