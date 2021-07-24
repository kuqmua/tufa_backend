use crate::get_project_information::get_providers_link_parts_from_mongo::get_providers_link_parts_from_mongo;
use config_lib::get_project_information::get_config::get_config_information::CONFIG;
use config_lib::get_project_information::get_user_credentials::get_user_credentials_information::USER_CREDENTIALS;

use std::collections::HashMap;

use crate::get_project_information::get_providers_json_local_data::get_providers_json_local_data;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn get_providers_link_parts(resource: &Resource) -> HashMap<String, Vec<String>> {
    let vec_of_link_parts_hashmap: HashMap<String, Vec<String>>;
    match resource {
        Resource::Local {
            path_to_provider_link_parts_folder,
            vec_of_provider_names,
            second_part_of_file_name,
            file_extension,
        } => {
            vec_of_link_parts_hashmap = get_providers_json_local_data(
                path_to_provider_link_parts_folder,
                vec_of_provider_names.to_vec(),
                second_part_of_file_name,
                file_extension,
            );
        }
        Resource::Mongodb {
            mongo_url,
            db_name_handle,
            db_collection_handle_second_part,
            db_collection_document_field_name_handle,
            vec_of_provider_names,
        } => {
            let mongo_url: String;
            if CONFIG.mongo_params.is_cloud {
                let mongo_cloud_first_handle_url_part =
                    &CONFIG.mongo_params.mongo_cloud_first_handle_url_part;
                let mongo_cloud_login =
                    &USER_CREDENTIALS.mongo_cloud_authorization.mongo_cloud_login;
                let mongo_cloud_second_handle_url_part =
                    &CONFIG.mongo_params.mongo_cloud_second_handle_url_part;
                let mongo_cloud_password = &USER_CREDENTIALS
                    .mongo_cloud_authorization
                    .mongo_cloud_password;
                let mongo_cloud_third_handle_url_part =
                    &CONFIG.mongo_params.mongo_cloud_third_handle_url_part;
                let mongo_cloud_cluster_name = &USER_CREDENTIALS
                    .mongo_cloud_authorization
                    .mongo_cloud_cluster_name;
                let mongo_cloud_fourth_handle_url_part =
                    &CONFIG.mongo_params.mongo_cloud_fourth_handle_url_part;
                let mongo_cloud_cluster_params = &USER_CREDENTIALS
                    .mongo_cloud_authorization
                    .mongo_cloud_cluster_params;
                mongo_url = format!(
                    "{}{}{}{}{}{}{}{}",
                    mongo_cloud_first_handle_url_part,
                    mongo_cloud_login,
                    mongo_cloud_second_handle_url_part,
                    mongo_cloud_password,
                    mongo_cloud_third_handle_url_part,
                    mongo_cloud_cluster_name,
                    mongo_cloud_fourth_handle_url_part,
                    mongo_cloud_cluster_params
                );
            } else {
                let mongo_own_first_handle_url_part =
                    &CONFIG.mongo_params.mongo_own_first_handle_url_part;
                let mongo_own_login = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_login;
                let mongo_own_second_handle_url_part =
                    &CONFIG.mongo_params.mongo_own_second_handle_url_part;
                let mongo_own_password =
                    &USER_CREDENTIALS.mongo_own_authorization.mongo_own_password;
                let mongo_own_third_handle_url_part =
                    &CONFIG.mongo_params.mongo_own_third_handle_url_part;
                let mongo_own_ip = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_ip;
                let mongo_own_fourth_handle_url_part =
                    &CONFIG.mongo_params.mongo_own_fourth_handle_url_part;
                let mongo_own_port = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_port;
                mongo_url = format!(
                    "{}{}{}{}{}{}{}{}",
                    mongo_own_first_handle_url_part,
                    mongo_own_login,
                    mongo_own_second_handle_url_part,
                    mongo_own_password,
                    mongo_own_third_handle_url_part,
                    mongo_own_ip,
                    mongo_own_fourth_handle_url_part,
                    mongo_own_port
                );
            }
            vec_of_link_parts_hashmap = get_providers_link_parts_from_mongo(
                mongo_url,
                CONFIG.mongo_params.db_name_handle.clone(),
                CONFIG.mongo_params.db_collection_handle_second_part.clone(),
                CONFIG
                    .mongo_params
                    .db_collection_document_field_name_handle
                    .clone(),
                CONFIG.params.vec_of_provider_names.clone(),
            );
        }
        Resource::PostgreSql => {
            todo!()
        }
    }
    vec_of_link_parts_hashmap
}
// #[derive(Clone, Debug, serde_derive::Deserialize, PartialEq, serde_derive::Serialize)]
pub enum Resource {
    Local {
        path_to_provider_link_parts_folder: String,
        vec_of_provider_names: Vec<String>,
        second_part_of_file_name: String,
        file_extension: String,
    },
    Mongodb {
        mongo_url: String,
        db_name_handle: String,
        db_collection_handle_second_part: String,
        db_collection_document_field_name_handle: String,
        vec_of_provider_names: Vec<String>,
    },
    PostgreSql,
}
