use std::collections::HashMap;

use config_lib::get_project_information::get_config::get_lazy_config_information::CONFIG;
use config_lib::get_project_information::get_user_credentials::get_lazy_user_credentials_information::USER_CREDENTIALS;
use config_lib::get_project_information::project_constants::get_config_provider_string_to_enum_struct;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use crate::get_project_information::get_providers_link_parts::get_providers_link_parts;
use crate::get_project_information::get_providers_link_parts::Resource;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_wrapper() -> Option<HashMap<String, Vec<String>>> {
    let mongo_url: String;
    if CONFIG.mongo_params.is_cloud {
        let mongo_cloud_first_handle_url_part = &CONFIG
            .mongo_params
            .enable_mongo_cloud_url_parts
            .mongo_cloud_first_handle_url_part;
        let mongo_cloud_login = &USER_CREDENTIALS.mongo_cloud_authorization.mongo_cloud_login;
        let mongo_cloud_second_handle_url_part = &CONFIG
            .mongo_params
            .enable_mongo_cloud_url_parts
            .mongo_cloud_second_handle_url_part;
        let mongo_cloud_password = &USER_CREDENTIALS
            .mongo_cloud_authorization
            .mongo_cloud_password;
        let mongo_cloud_third_handle_url_part = &CONFIG
            .mongo_params
            .enable_mongo_cloud_url_parts
            .mongo_cloud_third_handle_url_part;
        let mongo_cloud_cluster_name = &USER_CREDENTIALS
            .mongo_cloud_authorization
            .mongo_cloud_cluster_name;
        let mongo_cloud_fourth_handle_url_part = &CONFIG
            .mongo_params
            .enable_mongo_cloud_url_parts
            .mongo_cloud_fourth_handle_url_part;
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
        let mongo_own_first_handle_url_part = &CONFIG.mongo_params.mongo_own_first_handle_url_part;
        let mongo_own_login = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_login;
        let mongo_own_second_handle_url_part =
            &CONFIG.mongo_params.mongo_own_second_handle_url_part;
        let mongo_own_password = &USER_CREDENTIALS.mongo_own_authorization.mongo_own_password;
        let mongo_own_third_handle_url_part = &CONFIG.mongo_params.mongo_own_third_handle_url_part;
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
    let mut providers_string_into_enum_hashmap: HashMap<String, ProviderKind> =
        HashMap::with_capacity(CONFIG.params.vec_of_provider_names.len());
    let config_provider_string_to_enum_struct_hashmap = get_config_provider_string_to_enum_struct();
    for provider_name in &CONFIG.params.vec_of_provider_names {
        match config_provider_string_to_enum_struct_hashmap.get(provider_name) {
            Some(provider_kind_handle) => {
                providers_string_into_enum_hashmap
                    .insert(provider_name.to_string(), *provider_kind_handle);
            }
            None => {
                print_colorful_message(
                    None,
                    PrintType::Success,
                    file!().to_string(),
                    line!().to_string(),
                    format!("{} was disabled", provider_name),
                );
            }
        }
    }
    let providers_link_parts = get_providers_link_parts(&Resource::Mongodb {
        mongo_url,
        db_name_handle: CONFIG.mongo_params.providers_db_name_handle.to_string(),
        db_collection_handle_second_part: CONFIG
            .mongo_params
            .providers_db_collection_handle_second_part
            .to_string(),
        db_collection_document_field_name_handle: CONFIG
            .mongo_params
            .providers_db_collection_document_field_name_handle
            .to_string(),
        providers_string_into_enum_hashmap,
    })
    .await;
    if !providers_link_parts.is_empty() {
        Some(providers_link_parts)
    } else {
        print_colorful_message(
            None,
            PrintType::WarningLow,
            file!().to_string(),
            line!().to_string(),
            "providers_link_parts .is_empty".to_string(),
        );
        let providers_link_parts_local = get_providers_link_parts(&Resource::Local {
            path_to_provider_link_parts_folder: CONFIG
                .mongo_params
                .path_to_provider_link_parts_folder
                .to_string(),
            vec_of_provider_names: CONFIG.params.vec_of_provider_names.clone(),
            second_part_of_file_name: CONFIG
                .mongo_params
                .providers_db_collection_handle_second_part //why that in mongo_params?
                .to_string(),
            file_extension: CONFIG.mongo_params.log_file_extension.to_string(),
        })
        .await;
        if !providers_link_parts_local.is_empty() {
            Some(providers_link_parts_local)
        } else {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                "providers_link_parts_local.is_empty too".to_string(),
            );
            None
        }
    }
}
