use std::collections::HashMap;

use config_lib::get_project_information::get_config::get_lazy_config_information::CONFIG;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use crate::get_project_information::get_providers_link_parts::get_providers_link_parts;
use crate::get_project_information::get_providers_link_parts::Resource;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_wrapper() -> Option<HashMap<String, Vec<String>>> {
    let mongo_url: String;
    if CONFIG.mongo_params.mongo_is_cloud {
        // let mongo_cloud_first_handle_url_part = &CONFIG
        //     .mongo_params
        //     .enable_mongo_cloud_url_parts
        //     .mongo_cloud_first_handle_url_part;
        // let mongo_cloud_login = &CONFIG.mongo_cloud_authorization.mongo_cloud_login;
        // let mongo_cloud_second_handle_url_part = &CONFIG
        //     .mongo_params
        //     .enable_mongo_cloud_url_parts
        //     .mongo_cloud_second_handle_url_part;
        // let mongo_cloud_password = &CONFIG.mongo_cloud_authorization.mongo_cloud_password;
        // let mongo_cloud_third_handle_url_part = &CONFIG
        //     .mongo_params
        //     .enable_mongo_cloud_url_parts
        //     .mongo_cloud_third_handle_url_part;
        // let mongo_cloud_cluster_name = &CONFIG.mongo_cloud_authorization.mongo_cloud_cluster_name;
        // let mongo_cloud_fourth_handle_url_part = &CONFIG
        //     .mongo_params
        //     .enable_mongo_cloud_url_parts
        //     .mongo_cloud_fourth_handle_url_part;
        // let mongo_cloud_cluster_params =
        //     &CONFIG.mongo_cloud_authorization.mongo_cloud_cluster_params;
        // mongo_url = format!(
        //     "{}{}{}{}{}{}{}{}",
        //     mongo_cloud_first_handle_url_part,
        //     mongo_cloud_login,
        //     mongo_cloud_second_handle_url_part,
        //     mongo_cloud_password,
        //     mongo_cloud_third_handle_url_part,
        //     mongo_cloud_cluster_name,
        //     mongo_cloud_fourth_handle_url_part,
        //     mongo_cloud_cluster_params
        // );
        let mongo_first_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_first_handle_url_part;
        let mongo_login = &CONFIG.mongo_authorization.mongo_login;
        let mongo_second_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_second_handle_url_part;
        let mongo_password = &CONFIG.mongo_authorization.mongo_password;
        let mongo_third_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_third_handle_url_part;
        let mongo_ip = &CONFIG.mongo_authorization.mongo_ip;
        let mongo_fourth_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_fourth_handle_url_part;
        let mongo_port = &CONFIG.mongo_authorization.mongo_port;
        mongo_url = format!(
            "{}{}{}{}{}{}{}{}",
            mongo_first_handle_url_part,
            mongo_login,
            mongo_second_handle_url_part,
            mongo_password,
            mongo_third_handle_url_part,
            mongo_ip,
            mongo_fourth_handle_url_part,
            mongo_port
        );
    } else {
        let mongo_first_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_first_handle_url_part;
        let mongo_login = &CONFIG.mongo_authorization.mongo_login;
        let mongo_second_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_second_handle_url_part;
        let mongo_password = &CONFIG.mongo_authorization.mongo_password;
        let mongo_third_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_third_handle_url_part;
        let mongo_ip = &CONFIG.mongo_authorization.mongo_ip;
        let mongo_fourth_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_fourth_handle_url_part;
        let mongo_port = &CONFIG.mongo_authorization.mongo_port;
        let mongo_params = &CONFIG.mongo_authorization.mongo_params;
        mongo_url = format!(
            "{}{}{}{}{}{}{}{}",
            mongo_first_handle_url_part,
            mongo_login,
            mongo_second_handle_url_part,
            mongo_password,
            mongo_third_handle_url_part,
            mongo_ip,
            mongo_fourth_handle_url_part,
            mongo_port
        );
    }
    let providers_string_into_enum_hashmap: HashMap<String, ProviderKind> = ProviderKind::into_provider_string_name_provider_kind_hashmap();
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
