use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future::join_all;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::providers::providers_info::get_provider_link_parts_from_mongo::get_provider_link_parts_from_mongo;

pub async fn get_providers_link_parts_from_mongo(
    providers_string_into_enum_hashmap: HashMap<&'static str, ProviderKind>,
) -> HashMap<&'static str, Vec<String>> {
    let vec_of_link_parts_hashmap_under_arc =
        Arc::new(Mutex::new(HashMap::<&'static str, Vec<String>>::new()));
    let mut vec_of_tasks = Vec::with_capacity(providers_string_into_enum_hashmap.len());
    for provider_tuple in providers_string_into_enum_hashmap {
        let vec_of_link_parts_hashmap_under_arc_handle =
            Arc::clone(&vec_of_link_parts_hashmap_under_arc);
        if ProviderKind::get_string_name(provider_tuple.1) == provider_tuple.0 {
            if ProviderKind::is_enabled(provider_tuple.1) {
                vec_of_tasks.push(tokio::task::spawn(get_provider_link_parts_from_mongo(
                    provider_tuple.1,
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