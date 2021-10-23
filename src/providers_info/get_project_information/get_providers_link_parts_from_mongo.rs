use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future::join_all;

use config_lib::get_project_information::get_config::get_lazy_config_information::CONFIG;
use config_lib::get_project_information::project_constants::ARXIV_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::BIORXIV_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::GITHUB_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::HABR_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::MEDRXIV_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::REDDIT_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::TWITTER_NAME_TO_CHECK;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use crate::mongo_integration::mongo_get_provider_link_parts_as_bson_string::mongo_get_provider_link_parts_as_bson_string;

pub async fn get_providers_link_parts_from_mongo(
    mongo_url: String,
    db_name_handle: String,
    db_collection_handle_second_part: String,
    db_collection_document_field_name_handle: String,
    providers_string_into_enum_hashmap: HashMap<String, ProviderKind>,
) -> HashMap<String, Vec<String>> {
    let vec_of_link_parts_hashmap_under_arc =
        Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new()));
    let mut vec_of_tasks = Vec::with_capacity(providers_string_into_enum_hashmap.len());
    for provider_tuple in providers_string_into_enum_hashmap {
        let mongo_url_clone = mongo_url.clone();
        let db_name_handle_clone = db_name_handle.clone();
        let db_collection_handle_second_part_clone = db_collection_handle_second_part.clone();
        let db_collection_document_field_name_handle_clone =
            db_collection_document_field_name_handle.clone();
        let vec_of_link_parts_hashmap_under_arc_handle =
            Arc::clone(&vec_of_link_parts_hashmap_under_arc);
        if provider_tuple.0 == ARXIV_NAME_TO_CHECK {
            if CONFIG.enable_providers.enable_arxiv {
                vec_of_tasks.push(tokio::task::spawn(get_provider_link_parts_from_mongo(
                    mongo_url_clone,
                    db_name_handle_clone,
                    provider_tuple.0,
                    provider_tuple.1,
                    db_collection_handle_second_part_clone,
                    db_collection_document_field_name_handle_clone,
                    vec_of_link_parts_hashmap_under_arc_handle,
                )));
            }
        } else if provider_tuple.0 == BIORXIV_NAME_TO_CHECK {
            if CONFIG.enable_providers.enable_biorxiv {
                vec_of_tasks.push(tokio::task::spawn(get_provider_link_parts_from_mongo(
                    mongo_url_clone,
                    db_name_handle_clone,
                    provider_tuple.0,
                    provider_tuple.1,
                    db_collection_handle_second_part_clone,
                    db_collection_document_field_name_handle_clone,
                    vec_of_link_parts_hashmap_under_arc_handle,
                )));
            }
        } else if provider_tuple.0 == GITHUB_NAME_TO_CHECK {
            if CONFIG.enable_providers.enable_github {
                vec_of_tasks.push(tokio::task::spawn(get_provider_link_parts_from_mongo(
                    mongo_url_clone,
                    db_name_handle_clone,
                    provider_tuple.0,
                    provider_tuple.1,
                    db_collection_handle_second_part_clone,
                    db_collection_document_field_name_handle_clone,
                    vec_of_link_parts_hashmap_under_arc_handle,
                )));
            }
        } else if provider_tuple.0 == HABR_NAME_TO_CHECK {
            if CONFIG.enable_providers.enable_habr {
                vec_of_tasks.push(tokio::task::spawn(get_provider_link_parts_from_mongo(
                    mongo_url_clone,
                    db_name_handle_clone,
                    provider_tuple.0,
                    provider_tuple.1,
                    db_collection_handle_second_part_clone,
                    db_collection_document_field_name_handle_clone,
                    vec_of_link_parts_hashmap_under_arc_handle,
                )));
            }
        } else if provider_tuple.0 == MEDRXIV_NAME_TO_CHECK {
            if CONFIG.enable_providers.enable_medrxiv {
                vec_of_tasks.push(tokio::task::spawn(get_provider_link_parts_from_mongo(
                    mongo_url_clone,
                    db_name_handle_clone,
                    provider_tuple.0,
                    provider_tuple.1,
                    db_collection_handle_second_part_clone,
                    db_collection_document_field_name_handle_clone,
                    vec_of_link_parts_hashmap_under_arc_handle,
                )));
            }
        } else if provider_tuple.0 == REDDIT_NAME_TO_CHECK {
            if CONFIG.enable_providers.enable_reddit {
                vec_of_tasks.push(tokio::task::spawn(get_provider_link_parts_from_mongo(
                    mongo_url_clone,
                    db_name_handle_clone,
                    provider_tuple.0,
                    provider_tuple.1,
                    db_collection_handle_second_part_clone,
                    db_collection_document_field_name_handle_clone,
                    vec_of_link_parts_hashmap_under_arc_handle,
                )));
            }
        } else if provider_tuple.0 == TWITTER_NAME_TO_CHECK {
            if CONFIG.enable_providers.enable_twitter {
                vec_of_tasks.push(tokio::task::spawn(get_provider_link_parts_from_mongo(
                    mongo_url_clone,
                    db_name_handle_clone,
                    provider_tuple.0,
                    provider_tuple.1,
                    db_collection_handle_second_part_clone,
                    db_collection_document_field_name_handle_clone,
                    vec_of_link_parts_hashmap_under_arc_handle,
                )));
            }
        } else {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!("some different provider {}", provider_tuple.0),
            );
        }
    }
    let _ = join_all(vec_of_tasks).await;
    let vec_of_link_parts_hashmap = vec_of_link_parts_hashmap_under_arc.lock().unwrap().clone();

    vec_of_link_parts_hashmap
}

#[allow(clippy::too_many_arguments)]
async fn get_provider_link_parts_from_mongo(
    mongo_url_clone: String,
    db_name_handle_clone: String,
    provider_tuple_0: String,
    provider_tuple_1: ProviderKind,
    db_collection_handle_second_part_clone: String,
    db_collection_document_field_name_handle_clone: String,
    vec_of_link_parts_hashmap_under_arc_handle: Arc<Mutex<HashMap<String, Vec<String>>>>,
) {
    let result_getting_provider_link_parts = mongo_get_provider_link_parts_as_bson_string(
        &mongo_url_clone,
        &db_name_handle_clone,
        format!(
            "{}{}",
            provider_tuple_0, db_collection_handle_second_part_clone
        ),
        &db_collection_document_field_name_handle_clone,
        provider_tuple_1,
    ).await;
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
