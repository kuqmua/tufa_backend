use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future::join_all;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::providers::providers_info::get_provider_link_parts_from_mongo::get_provider_link_parts_from_mongo;

pub async fn get_providers_link_parts_from_mongo() -> HashMap<ProviderKind, Vec<String>> {
    let mongo_initialization_provider_kind_vec =
        ProviderKind::get_mongo_initialization_provider_kind_vec();
    let vec_of_link_parts_hashmap_under_arc =
        Arc::new(Mutex::new(HashMap::<ProviderKind, Vec<String>>::new()));
    let mut vec_of_tasks = Vec::with_capacity(mongo_initialization_provider_kind_vec.len());
    for provider_kind in mongo_initialization_provider_kind_vec {
        let vec_of_link_parts_hashmap_under_arc_handle =
            Arc::clone(&vec_of_link_parts_hashmap_under_arc);
        vec_of_tasks.push(tokio::task::spawn(get_provider_link_parts_from_mongo(
            provider_kind,
            vec_of_link_parts_hashmap_under_arc_handle,
        )));
    }
    let _ = join_all(vec_of_tasks).await;
    let vec_of_link_parts_hashmap = vec_of_link_parts_hashmap_under_arc.lock().unwrap().clone();

    vec_of_link_parts_hashmap
}
