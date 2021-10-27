use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future::join_all;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::mongo_integration::mongo_get_provider_link_parts_as_bson_string::mongo_get_provider_link_parts_as_bson_string;

pub async fn get_providers_link_parts_from_mongo(
    db_name_handle: String,
    db_collection_document_field_name_handle: String,
    providers_string_into_enum_hashmap: HashMap<&'static str, ProviderKind>,
) -> HashMap<&'static str, Vec<String>> {
    let vec_of_link_parts_hashmap_under_arc =
        Arc::new(Mutex::new(HashMap::<&'static str, Vec<String>>::new()));
    let mut vec_of_tasks = Vec::with_capacity(providers_string_into_enum_hashmap.len());
    for provider_tuple in providers_string_into_enum_hashmap {
        let db_name_handle_clone = db_name_handle.clone();
        let db_collection_document_field_name_handle_clone =
            db_collection_document_field_name_handle.clone();
        let vec_of_link_parts_hashmap_under_arc_handle =
            Arc::clone(&vec_of_link_parts_hashmap_under_arc);
        if ProviderKind::get_string_name(provider_tuple.1) == provider_tuple.0 {
            if ProviderKind::is_enabled(provider_tuple.1) {
                vec_of_tasks.push(tokio::task::spawn(get_provider_link_parts_from_mongo(
                    db_name_handle_clone,
                    provider_tuple.0,
                    provider_tuple.1,
                    db_collection_document_field_name_handle_clone,
                    vec_of_link_parts_hashmap_under_arc_handle,
                )));
            }
        } else {
            println!("different provider kinf string name (remove after migrating into enums)")
        }
    }
    let _ = join_all(vec_of_tasks).await;
    let vec_of_link_parts_hashmap = vec_of_link_parts_hashmap_under_arc.lock().unwrap().clone();

    vec_of_link_parts_hashmap
}

#[allow(clippy::too_many_arguments)]
async fn get_provider_link_parts_from_mongo(
    db_name_handle_clone: String,
    provider_tuple_0: &'static str,
    provider_tuple_1: ProviderKind,
    db_collection_document_field_name_handle_clone: String,
    vec_of_link_parts_hashmap_under_arc_handle: Arc<Mutex<HashMap<&'static str, Vec<String>>>>,
) {
    let result_getting_provider_link_parts = ProviderKind::mongo_get_provider_link_parts_as_bson_string(provider_tuple_1).await;
    match result_getting_provider_link_parts {
        Ok(provider_link_parts) => {
            let mut vec_of_link_parts_hashmap_under_arc_handle_locked =
                vec_of_link_parts_hashmap_under_arc_handle.lock().unwrap();
            vec_of_link_parts_hashmap_under_arc_handle_locked
                .insert(provider_tuple_0, provider_link_parts);
        }
        Err(e) => {
            println!("result_getting_provider_link_parts error {:#?}", e);
        }
    }
}
