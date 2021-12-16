use std::collections::HashMap;
use std::fs;

use diesel::prelude::*;

use crate::config_mods::lazy_static_config::CONFIG;
use crate::providers::get_providers_json_local_data_processed_error::GetProvidersJsonLocalDataProcessedError;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;
use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;

use crate::init_dbs::init_mongo::init_mongo;
use crate::init_dbs::init_mongo::MongoInitDbError;

use crate::init_dbs::init_postgres::init_postgres;
use crate::init_dbs::init_postgres::PostgresInitDbError;

use tokio::task;

use async_std::fs::File;
use async_std::prelude::*;
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
    //
    // let mut vec_of_link_parts_hashmap: HashMap<
    //         ProviderKind,
    //         Result<Result<Vec<String>, serde_json::Error>, std::io::Error>,
    //     > = HashMap::with_capacity(ProviderKind::get_enabled_providers_vec().len());
        // //todo: do it async in parallel
        // let mut futures_vec = Vec::with_capacity(ProviderKind::get_enabled_providers_vec().len());
        // // HashMap<ProviderKind, Result<Result<Vec<String>, serde_json::Error>, std::io::Error>>
        // for pk in ProviderKind::get_enabled_providers_vec() {
        //     futures_vec.push(task::spawn(async move {
        //         match File::open(pk.get_init_local_data_file_path()).await {
        //             Err(e) => {
        //                 (pk, Err(e))
        //             }
        //             Ok(file) => {
        //                 let mut buf = Vec::new();
        //                 file.read_to_end(&mut buf).await?;
        //                 let s = String::from_utf8_lossy(&buf);
        //                 // let file_content_from_str_result: Result<
        //                 //     ProvidersInitJsonSchema,
        //                 //     serde_json::Error,
        //                 // > = serde_json::from_str(&file_content);
        //                 // match file_content_from_str_result {
        //                 //     Ok(file_content_as_struct) => {
        //                 //         let mut vec_of_link_parts: Vec<String> =
        //                 //             Vec::with_capacity(file_content_as_struct.data.len());
        //                 //         for link_part in file_content_as_struct.data {
        //                 //             vec_of_link_parts.push(link_part)
        //                 //         }
        //                 //         (pk, Ok(Ok(vec_of_link_parts)))
        //                 //     }
        //                 //     Err(e) => {
        //                 //         (pk, Ok(Err(e)))
        //                 //     }
        //                 // }
        //             }
        //         }
        //     }))
            
        // }
        // // let result_vec = futures_vec.join_all().await;
        // let result = tokio::join!(futures_vec);

        // // .insert(pk, Ok(Ok(vec_of_link_parts)));
    //
    let (success_hashmap, errors_hashmap) = ProviderKind::get_providers_json_local_data_processed();
    if !errors_hashmap.is_empty() {
        return Err(InitDbsError::GetProvidersJsonLocalData(errors_hashmap));
    }
    providers_json_local_data_hashmap = success_hashmap.clone();
    providers_json_local_data_hashmap_clone = success_hashmap;
    let (mongo_insert_data_option_result, postgres_insert_data_option_result) = tokio::join!(
        async {
            if CONFIG.mongo_enable_initialization {
                return Some(init_mongo(providers_json_local_data_hashmap).await);
            }
            None
        },
        async {
            if !CONFIG.postgres_enable_initialization {
                return Some(init_postgres(providers_json_local_data_hashmap_clone).await);
            }
            None
        }
    );
    if let Some(result) = mongo_insert_data_option_result {
        if let Err(err) = result {
            match err {
                MongoInitDbError::ClientOptionsParse(e) => return Err(InitDbsError::MongoClientOptionsParse(e)),
                MongoInitDbError::ClientWithOptions(e) => return Err(InitDbsError::MongoClientWithOptions(e)),
                MongoInitDbError::CollectionCountDocuments((pk, e)) => return Err(InitDbsError::MongoCollectionCountDocuments((pk, e))),
                MongoInitDbError::CollectionIsNotEmpty((pk, documents_number)) => return Err(InitDbsError::MongoCollectionIsNotEmpty((pk, documents_number))),
                MongoInitDbError::CollectionInsertMany((pk, e)) => return Err(InitDbsError::MongoCollectionInsertMany((pk, e)))
            }
        }
    }
    if let Some(result) = postgres_insert_data_option_result {
        if let Err(err) = result {
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
    }
    Ok(())
}