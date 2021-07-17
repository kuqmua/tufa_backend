use config_lib::get_project_information::get_config::get_config_information::CONFIG;
use config_lib::get_project_information::project_constants::ARXIV_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::BIORXIV_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::GITHUB_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::HABR_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::MEDRXIV_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::REDDIT_NAME_TO_CHECK;
use config_lib::get_project_information::project_constants::TWITTER_NAME_TO_CHECK;

use mongo_integration::mongo_get_provider_link_parts_as_bson_string::mongo_get_provider_link_parts_as_bson_string;
use std::collections::HashMap;

pub fn get_providers_link_parts_from_mongo(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_handle_second_part: &str,
    db_collection_document_field_name_handle: &str,
    vec_of_provider_names: Vec<String>,
) -> HashMap<String, Vec<String>> {
    let mut vec_of_link_parts_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    for provider_name in vec_of_provider_names {
        if provider_name == ARXIV_NAME_TO_CHECK {
            if CONFIG.enable_providers.enable_arxiv {
                let result_getting_provider_link_parts =
                    mongo_get_provider_link_parts_as_bson_string(
                        mongo_url,
                        db_name_handle,
                        format!("{}{}", provider_name, db_collection_handle_second_part),
                        db_collection_document_field_name_handle,
                    );
                match result_getting_provider_link_parts {
                    Ok(provider_link_parts) => {
                        vec_of_link_parts_hashmap.insert(provider_name, provider_link_parts);
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
                        mongo_url,
                        db_name_handle,
                        format!("{}{}", provider_name, db_collection_handle_second_part),
                        db_collection_document_field_name_handle,
                    );
                match result_getting_provider_link_parts {
                    Ok(provider_link_parts) => {
                        vec_of_link_parts_hashmap.insert(provider_name, provider_link_parts);
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
                        mongo_url,
                        db_name_handle,
                        format!("{}{}", provider_name, db_collection_handle_second_part),
                        db_collection_document_field_name_handle,
                    );
                match result_getting_provider_link_parts {
                    Ok(provider_link_parts) => {
                        vec_of_link_parts_hashmap.insert(provider_name, provider_link_parts);
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
                        mongo_url,
                        db_name_handle,
                        format!("{}{}", provider_name, db_collection_handle_second_part),
                        db_collection_document_field_name_handle,
                    );
                match result_getting_provider_link_parts {
                    Ok(provider_link_parts) => {
                        vec_of_link_parts_hashmap.insert(provider_name, provider_link_parts);
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
                        mongo_url,
                        db_name_handle,
                        format!("{}{}", provider_name, db_collection_handle_second_part),
                        db_collection_document_field_name_handle,
                    );
                match result_getting_provider_link_parts {
                    Ok(provider_link_parts) => {
                        vec_of_link_parts_hashmap.insert(provider_name, provider_link_parts);
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
                        mongo_url,
                        db_name_handle,
                        format!("{}{}", provider_name, db_collection_handle_second_part),
                        db_collection_document_field_name_handle,
                    );
                match result_getting_provider_link_parts {
                    Ok(provider_link_parts) => {
                        vec_of_link_parts_hashmap.insert(provider_name, provider_link_parts);
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
                        mongo_url,
                        db_name_handle,
                        format!("{}{}", provider_name, db_collection_handle_second_part),
                        db_collection_document_field_name_handle,
                    );
                match result_getting_provider_link_parts {
                    Ok(provider_link_parts) => {
                        vec_of_link_parts_hashmap.insert(provider_name, provider_link_parts);
                    }
                    Err(e) => {
                        println!("result_getting_provider_link_parts error {:#?}", e);
                    }
                }
            }
        } else {
            println!("f4530603")
        }
    }
    vec_of_link_parts_hashmap
}
