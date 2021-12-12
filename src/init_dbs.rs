use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::mongo_integration::mongo_insert_data::{mongo_insert_data, PutDataInMongoResult};

use crate::postgres_integration::models::insertable::queryable_link_part::InsertableLinkPart;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use crate::providers::get_providers_json_local_data_processed_error::GetProvidersJsonLocalDataProcessedError;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;
use crate::postgres_integration::schema::providers_link_parts::dsl::*;

#[derive(Debug)]
pub enum InitDbsError {
    GetProvidersJsonLocalData(HashMap<ProviderKind, GetProvidersJsonLocalDataProcessedError>),
    MongoInsertDataPartial, //todo: add values in here
    MongoInsertDataFailure,
    PostgresLoadingProvidersLinkParts(diesel::result::Error),
    PostgresProvidersLinkPartsIsNotEmpty(Vec<QueryableLinkPart>),
    PostgresInsertPosts(diesel::result::Error),
    PostgresEstablishConnection(ConnectionError),
}

#[derive(Debug)]
pub enum PostgresInitDbError {
    LoadingProvidersLinkParts(diesel::result::Error),
    ProvidersLinkPartsIsNotEmpty(Vec<QueryableLinkPart>),
    InsertPosts(diesel::result::Error),
    EstablishConnection(ConnectionError),
}

#[deny(clippy::indexing_slicing)]
pub async fn init_dbs() -> Result<(), InitDbsError> {
    let providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>;
    let (success_hashmap, errors_hashmap) = ProviderKind::get_providers_json_local_data_processed();
    if !errors_hashmap.is_empty() {
        return Err(InitDbsError::GetProvidersJsonLocalData(errors_hashmap));
    }
    providers_json_local_data_hashmap = success_hashmap;
    let (mongo_insert_data_option_result, postgres_insert_data_option_result) = tokio::join!(
        async {
            if !CONFIG.mongo_enable_initialization {
                return None;
            }
            Some(
                mongo_insert_data(
                    &CONFIG.mongo_providers_logs_db_name,
                    providers_json_local_data_hashmap.clone(), //clone coz move in postgres part
                )
                .await,
            )
        },
        async {
            if !CONFIG.postgres_enable_initialization {
                return None;
            }
            let result_postgres_establish_connection =
                PgConnection::establish(&postgres_get_db_url());
            match result_postgres_establish_connection {
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
                            let insertion_result = InsertableLinkPart::insert_into_postgres(
                                &pg_connection,
                                InsertableLinkPart {
                                    link_part: "post_title", //todo
                                },
                            );
                            match insertion_result {
                                Err(e) => Some(PostgresInitDbError::InsertPosts(e)),
                                Ok(_) => None,
                            }
                        }
                    }
                }
            }
        }
    );
    if let Some(result) = mongo_insert_data_option_result {
        match result {
            PutDataInMongoResult::Success => (),
            PutDataInMongoResult::PartialSuccess => {
                return Err(InitDbsError::MongoInsertDataPartial)
            }
            PutDataInMongoResult::Failure => return Err(InitDbsError::MongoInsertDataFailure),
        }
    }
    if let Some(result) = postgres_insert_data_option_result {
        println!("todo postgres_insert_data_option_result");
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
