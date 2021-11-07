use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::providers::provider_kind_enum::ProviderKind;

#[allow(clippy::too_many_arguments)]
pub async fn get_provider_link_parts_from_mongo(
    provider_kind: ProviderKind,
    vec_of_link_parts_hashmap_under_arc_handle: Arc<Mutex<HashMap<ProviderKind, Vec<String>>>>,
) {
    let result_getting_provider_link_parts =
        ProviderKind::mongo_get_provider_link_parts_as_bson_string(provider_kind).await;
    match result_getting_provider_link_parts {
        Ok(option_provider_link_parts) => {
            if let Some(provider_link_parts) = option_provider_link_parts {
                let mut vec_of_link_parts_hashmap_under_arc_handle_locked =
                    vec_of_link_parts_hashmap_under_arc_handle.lock().unwrap();
                vec_of_link_parts_hashmap_under_arc_handle_locked
                    .insert(provider_kind, provider_link_parts);
            }
        }
        Err(e) => {
            println!("result_getting_provider_link_parts error {:#?}", e);
        }
    }
}
