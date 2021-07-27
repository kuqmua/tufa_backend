use config_lib::get_project_information::get_config::get_config_information::CONFIG;
use config_lib::get_project_information::project_constants::ARXIV_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::BIORXIV_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::GITHUB_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::HABR_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::MEDRXIV_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::REDDIT_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::TWITTER_NAME_TO_CHECK;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use mongo_integration::mongo_get_provider_link_parts_as_bson_string::mongo_get_provider_link_parts_as_bson_string;
use std::collections::HashMap;

use std::thread;
use std::thread::JoinHandle;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use std::sync::{Arc, Mutex};

pub fn get_providers_link_parts_from_mongo(
    mongo_url: String,
    db_name_handle: String,
    db_collection_handle_second_part: String,
    db_collection_document_field_name_handle: String,
    vec_of_provider_names: Vec<String>,
) -> HashMap<String, Vec<String>> {
    let mut threads_vec: Vec<JoinHandle<()>> = Vec::with_capacity(vec_of_provider_names.len());
    let vec_of_link_parts_hashmap_under_arc =
        Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new()));
    for provider_name in vec_of_provider_names {
        let mongo_url_clone = mongo_url.clone();
        let db_name_handle_clone = db_name_handle.clone();
        let db_collection_handle_second_part_clone = db_collection_handle_second_part.clone();
        let db_collection_document_field_name_handle_clone =
            db_collection_document_field_name_handle.clone();
        let vec_of_link_parts_hashmap_under_arc_handle =
            Arc::clone(&vec_of_link_parts_hashmap_under_arc);
        threads_vec.push(thread::spawn(move || {
            if provider_name == ARXIV_NAME_TO_CHECK {
                if CONFIG.enable_providers.enable_arxiv {
                    let result_getting_provider_link_parts =
                        mongo_get_provider_link_parts_as_bson_string(
                            &mongo_url_clone,
                            &db_name_handle_clone,
                            format!(
                                "{}{}",
                                provider_name, db_collection_handle_second_part_clone
                            ),
                            &db_collection_document_field_name_handle_clone,
                            ProviderKind::Arxiv,
                        );
                    match result_getting_provider_link_parts {
                        Ok(provider_link_parts) => {
                            let mut vec_of_link_parts_hashmap_under_arc_handle_locked =
                                vec_of_link_parts_hashmap_under_arc_handle.lock().unwrap();
                            vec_of_link_parts_hashmap_under_arc_handle_locked
                                .insert(provider_name, provider_link_parts);
                        }
                        Err(e) => {
                            println!("result_getting_provider_link_parts error {:#?}", e);
                        }
                    }
                }
            } else if provider_name == BIORXIV_NAME_TO_CHECK {
                if CONFIG.enable_providers.enable_biorxiv {
                    let result_getting_provider_link_parts =
                        mongo_get_provider_link_parts_as_bson_string(
                            &mongo_url_clone,
                            &db_name_handle_clone,
                            format!(
                                "{}{}",
                                provider_name, db_collection_handle_second_part_clone
                            ),
                            &db_collection_document_field_name_handle_clone,
                            ProviderKind::Biorxiv,
                        );
                    match result_getting_provider_link_parts {
                        Ok(provider_link_parts) => {
                            let mut vec_of_link_parts_hashmap_under_arc_handle_locked =
                                vec_of_link_parts_hashmap_under_arc_handle.lock().unwrap();
                            vec_of_link_parts_hashmap_under_arc_handle_locked
                                .insert(provider_name, provider_link_parts);
                        }
                        Err(e) => {
                            println!("result_getting_provider_link_parts error {:#?}", e);
                        }
                    }
                }
            } else if provider_name == GITHUB_NAME_TO_CHECK {
                if CONFIG.enable_providers.enable_github {
                    let result_getting_provider_link_parts =
                        mongo_get_provider_link_parts_as_bson_string(
                            &mongo_url_clone,
                            &db_name_handle_clone,
                            format!(
                                "{}{}",
                                provider_name, db_collection_handle_second_part_clone
                            ),
                            &db_collection_document_field_name_handle_clone,
                            ProviderKind::Github,
                        );
                    match result_getting_provider_link_parts {
                        Ok(provider_link_parts) => {
                            let mut vec_of_link_parts_hashmap_under_arc_handle_locked =
                                vec_of_link_parts_hashmap_under_arc_handle.lock().unwrap();
                            vec_of_link_parts_hashmap_under_arc_handle_locked
                                .insert(provider_name, provider_link_parts);
                        }
                        Err(e) => {
                            println!("result_getting_provider_link_parts error {:#?}", e);
                        }
                    }
                }
            } else if provider_name == HABR_NAME_TO_CHECK {
                if CONFIG.enable_providers.enable_habr {
                    let result_getting_provider_link_parts =
                        mongo_get_provider_link_parts_as_bson_string(
                            &mongo_url_clone,
                            &db_name_handle_clone,
                            format!(
                                "{}{}",
                                provider_name, db_collection_handle_second_part_clone
                            ),
                            &db_collection_document_field_name_handle_clone,
                            ProviderKind::Habr,
                        );
                    match result_getting_provider_link_parts {
                        Ok(provider_link_parts) => {
                            let mut vec_of_link_parts_hashmap_under_arc_handle_locked =
                                vec_of_link_parts_hashmap_under_arc_handle.lock().unwrap();
                            vec_of_link_parts_hashmap_under_arc_handle_locked
                                .insert(provider_name, provider_link_parts);
                        }
                        Err(e) => {
                            println!("result_getting_provider_link_parts error {:#?}", e);
                        }
                    }
                }
            } else if provider_name == MEDRXIV_NAME_TO_CHECK {
                if CONFIG.enable_providers.enable_medrxiv {
                    let result_getting_provider_link_parts =
                        mongo_get_provider_link_parts_as_bson_string(
                            &mongo_url_clone,
                            &db_name_handle_clone,
                            format!(
                                "{}{}",
                                provider_name, db_collection_handle_second_part_clone
                            ),
                            &db_collection_document_field_name_handle_clone,
                            ProviderKind::Medrxiv,
                        );
                    match result_getting_provider_link_parts {
                        Ok(provider_link_parts) => {
                            let mut vec_of_link_parts_hashmap_under_arc_handle_locked =
                                vec_of_link_parts_hashmap_under_arc_handle.lock().unwrap();
                            vec_of_link_parts_hashmap_under_arc_handle_locked
                                .insert(provider_name, provider_link_parts);
                        }
                        Err(e) => {
                            println!("result_getting_provider_link_parts error {:#?}", e);
                        }
                    }
                }
            } else if provider_name == REDDIT_NAME_TO_CHECK {
                if CONFIG.enable_providers.enable_reddit {
                    let result_getting_provider_link_parts =
                        mongo_get_provider_link_parts_as_bson_string(
                            &mongo_url_clone,
                            &db_name_handle_clone,
                            format!(
                                "{}{}",
                                provider_name, db_collection_handle_second_part_clone
                            ),
                            &db_collection_document_field_name_handle_clone,
                            ProviderKind::Reddit,
                        );
                    match result_getting_provider_link_parts {
                        Ok(provider_link_parts) => {
                            let mut vec_of_link_parts_hashmap_under_arc_handle_locked =
                                vec_of_link_parts_hashmap_under_arc_handle.lock().unwrap();
                            vec_of_link_parts_hashmap_under_arc_handle_locked
                                .insert(provider_name, provider_link_parts);
                        }
                        Err(e) => {
                            println!("result_getting_provider_link_parts error {:#?}", e);
                        }
                    }
                }
            } else if provider_name == TWITTER_NAME_TO_CHECK {
                if CONFIG.enable_providers.enable_twitter {
                    let result_getting_provider_link_parts =
                        mongo_get_provider_link_parts_as_bson_string(
                            &mongo_url_clone,
                            &db_name_handle_clone,
                            format!(
                                "{}{}",
                                provider_name, db_collection_handle_second_part_clone
                            ),
                            &db_collection_document_field_name_handle_clone,
                            ProviderKind::Twitter,
                        );
                    match result_getting_provider_link_parts {
                        Ok(provider_link_parts) => {
                            let mut vec_of_link_parts_hashmap_under_arc_handle_locked =
                                vec_of_link_parts_hashmap_under_arc_handle.lock().unwrap();
                            vec_of_link_parts_hashmap_under_arc_handle_locked
                                .insert(provider_name, provider_link_parts);
                        }
                        Err(e) => {
                            println!("result_getting_provider_link_parts error {:#?}", e);
                        }
                    }
                }
            } else {
                print_colorful_message(
                    None,
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("some different provider {}", provider_name),
                );
            }
        }));
    }
    for i in threads_vec {
        i.join().unwrap();
    }
    let vec_of_link_parts_hashmap = vec_of_link_parts_hashmap_under_arc.lock().unwrap().clone();
    vec_of_link_parts_hashmap
}
