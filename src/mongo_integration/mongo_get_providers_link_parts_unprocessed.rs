use std::collections::HashMap;

use mongodb::{
    bson::{Document},
    options::ClientOptions,
    Client,
};

use crate::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector;
use crate::{
    config_mods::lazy_static_config::CONFIG, traits::provider_kind_trait::ProviderKindTrait,
};

use crate::{
    mongo_integration::mongo_get_db_url::mongo_get_db_url,
    providers::provider_kind_enum::ProviderKind,
};

use std::sync::{Arc, Mutex};

use futures::future::join_all;

use super::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorError;

#[deny(clippy::indexing_slicing)] //, clippy::unwrap_used
pub async fn mongo_get_providers_link_parts_unprocessed(
) -> Result<HashMap<ProviderKind, Result<Vec<String>, MongoGetDocumentsAsStringVectorError>>, mongodb::error::Error>
{
    //todo: write without arc - removing unwrap
    let client_options = ClientOptions::parse(mongo_get_db_url()).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&CONFIG.mongo_providers_logs_db_name);
    let vec_collection_names = db.list_collection_names(None).await?;
    let vec_provider_kind_with_collection_names_under_arc = Arc::new(Mutex::new(HashMap::<
        ProviderKind,
        Result<Vec<String>, MongoGetDocumentsAsStringVectorError>,
    >::new()));
    let mut vec_of_tasks = Vec::with_capacity(ProviderKind::get_enabled_providers_vec().len());
    for provider_kind in ProviderKind::get_enabled_providers_vec() {
        let vec_provider_kind_with_collection_names_under_arc_handle =
            Arc::clone(&vec_provider_kind_with_collection_names_under_arc);
        let collection_name = provider_kind.get_mongo_log_collection_name();
        let collection = db.collection::<Document>(&collection_name);
        if vec_collection_names.contains(&collection_name) {
            vec_of_tasks.push(tokio::task::spawn(async move {
                let result_vec_of_strings = mongo_get_documents_as_string_vector(
                    collection,
                    &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle,
                    ProviderKind::get_mongo_provider_link_parts_aggregation(&provider_kind),
                )
                .await;
                let mut vec_provider_kind_with_collection_names_under_arc_handle_locked =
                    vec_provider_kind_with_collection_names_under_arc_handle
                        .lock()
                        .unwrap();
                match result_vec_of_strings {
                    Ok(vec_of_strings) => {
                        vec_provider_kind_with_collection_names_under_arc_handle_locked
                            .insert(provider_kind, Ok(vec_of_strings));
                    }
                    Err(e) => {
                        vec_provider_kind_with_collection_names_under_arc_handle_locked
                            .insert(provider_kind, Err(e));
                    }
                }
            }));
        } else {
            let mut vec_provider_kind_with_collection_names_under_arc_handle_locked =
                vec_provider_kind_with_collection_names_under_arc_handle
                    .lock()
                    .unwrap();
            vec_provider_kind_with_collection_names_under_arc_handle_locked
                .insert(provider_kind, Ok(Vec::<String>::new()));
        }
    }
    let _ = join_all(vec_of_tasks).await;
    let vec_provider_kind_with_collection_names = vec_provider_kind_with_collection_names_under_arc
        .lock()
        .unwrap()
        .drain()
        .collect();
    Ok(vec_provider_kind_with_collection_names)
}
