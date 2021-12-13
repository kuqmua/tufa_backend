use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::postgres_integration::models::insertable::insertable_link_part::InsertableLinkPart;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::providers::get_providers_json_local_data_processed_error::GetProvidersJsonLocalDataProcessedError;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;
use crate::postgres_integration::schema::providers_link_parts::dsl::*;
//
#[derive(Debug)]
pub enum InitDbsError {
    GetProvidersJsonLocalData(HashMap<ProviderKind, GetProvidersJsonLocalDataProcessedError>),
    MongoClientOptionsParse(mongodb::error::Error),
    MongoClientWithOptions(mongodb::error::Error),
    MongoCollectionCountDocuments(mongodb::error::Error),
    MongoCollectionIsNotEmpty((ProviderKind, u64)),
    MongoCollectionInsertMany(mongodb::error::Error),
    PostgresLoadingProvidersLinkParts(diesel::result::Error),
    PostgresProvidersLinkPartsIsNotEmpty(Vec<QueryableLinkPart>),
    PostgresInsertPosts(diesel::result::Error),
    PostgresEstablishConnection(ConnectionError),
}

#[derive(Debug)]
pub enum MongoInitDbError {
    ClientOptionsParse(mongodb::error::Error),
    ClientWithOptions(mongodb::error::Error),
    CollectionCountDocuments(mongodb::error::Error),
    CollectionIsNotEmpty((ProviderKind, u64)),
    CollectionInsertMany(mongodb::error::Error),
}

#[derive(Debug)]
pub enum PostgresInitDbError {
    LoadingProvidersLinkParts(diesel::result::Error),
    ProvidersLinkPartsIsNotEmpty(Vec<QueryableLinkPart>),
    InsertPosts(diesel::result::Error),
    EstablishConnection(ConnectionError),
}

////////////////////////////////
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

////////////////////////////////

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs() -> Result<(), InitDbsError> {
    let providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>;
    let providers_json_local_data_hashmap_clone: HashMap<ProviderKind, Vec<String>>;
    let (success_hashmap, errors_hashmap) = ProviderKind::get_providers_json_local_data_processed();
    if !errors_hashmap.is_empty() {
        return Err(InitDbsError::GetProvidersJsonLocalData(errors_hashmap));
    }
    providers_json_local_data_hashmap = success_hashmap.clone();
    providers_json_local_data_hashmap_clone = success_hashmap;
    let (mongo_insert_data_option_result, postgres_insert_data_option_result) = tokio::join!(
        async {
            if !CONFIG.mongo_enable_initialization {
                return None;//todo: remove option into result
            }
            let client_options_result = ClientOptions::parse(&mongo_get_db_url()).await;
            match client_options_result {
                Err(e) => {
                    return Some(MongoInitDbError::ClientOptionsParse(e));
                },
                Ok(client_options) => {
                    match Client::with_options(client_options) {
                        Err(e) => return Some(MongoInitDbError::ClientWithOptions(e)),
                        Ok(client) => {
                            let db = client.database(&CONFIG.mongo_providers_logs_db_name);
                            for (pk, _) in &providers_json_local_data_hashmap {
                                let collection = db.collection::<Document>(&format!("{}",pk));
                                match collection.count_documents(None, None).await {//todo filter
                                    Err(e) => return Some(MongoInitDbError::CollectionCountDocuments(e)),
                                    Ok(documents_number) => {
                                        if documents_number > 0 {
                                            return Some(MongoInitDbError::CollectionIsNotEmpty((*pk, documents_number)));
                                        }
                                    },
                                }
                            }
                            for (pk, data_vec) in providers_json_local_data_hashmap {
                                let collection = db.collection(&format!("{}",pk));
                                let docs: Vec<Document> = data_vec.iter().map(|data| doc! { &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: data }).collect();
                                match collection.insert_many(docs, None).await {
                                    Err(e) => return Some(MongoInitDbError::CollectionInsertMany(e)),
                                    Ok(_) => (),
                                }
                            }
                            None//todo: remove option into result
                        },
                    }
                },
            }
        },
        async {
            if !CONFIG.postgres_enable_initialization {
                return None;//todo: remove option into result
            }
            match PgConnection::establish(&postgres_get_db_url()) {
                Err(e) => Some(PostgresInitDbError::EstablishConnection(e)),
                Ok(pg_connection) => {
                    let result = providers_link_parts
                        // .filter()
                        // .limit(5)
                        .load::<QueryableLinkPart>(&pg_connection);
                    match result {
                        Err(e) => Some(PostgresInitDbError::LoadingProvidersLinkParts(e)),
                        Ok(vec) => {
                            if !vec.is_empty() {
                                return Some(PostgresInitDbError::ProvidersLinkPartsIsNotEmpty(
                                    vec,
                                ));
                            }
                            let mut posts_vec: Vec<InsertableLinkPart> =
                                Vec::with_capacity(providers_json_local_data_hashmap_clone.len());
                            for (provider_kind_handle, data_vec) in
                                providers_json_local_data_hashmap_clone
                            {
                                for data in data_vec {
                                    posts_vec.push(InsertableLinkPart {
                                        provider_kind: format!("{}", provider_kind_handle.clone()),
                                        link_part: data.clone(),
                                    });
                                }
                            }
                            let insertion_result = InsertableLinkPart::insert_vec_into_postgres(
                                &pg_connection,
                                posts_vec,
                            );
                            match insertion_result {
                                Err(e) => Some(PostgresInitDbError::InsertPosts(e)),
                                Ok(_) => None,//todo: remove option into result
                            }
                        }
                    }
                }
            }
        }
    );
    if let Some(result) = mongo_insert_data_option_result {
        match result {
            MongoInitDbError::ClientOptionsParse(e) => return Err(InitDbsError::MongoClientOptionsParse(e)),
            MongoInitDbError::ClientWithOptions(e) => return Err(InitDbsError::MongoClientWithOptions(e)),
            MongoInitDbError::CollectionCountDocuments(e) => return Err(InitDbsError::MongoCollectionCountDocuments(e)),
            MongoInitDbError::CollectionIsNotEmpty((pk, documents_number)) => return Err(InitDbsError::MongoCollectionIsNotEmpty((pk, documents_number))),
            MongoInitDbError::CollectionInsertMany(e) => return Err(InitDbsError::MongoCollectionInsertMany(e))
        }
    }
    if let Some(result) = postgres_insert_data_option_result {
        match result {
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
