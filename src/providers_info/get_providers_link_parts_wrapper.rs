use std::collections::HashMap;

use crate::config_mods::config::CONFIG;
use crate::providers_info::provider_kind_enum::ProviderKind;

use crate::providers_info::get_project_information::get_providers_link_parts::get_providers_link_parts;
use crate::providers_info::get_project_information::get_providers_link_parts::Resource;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_wrapper() -> Option<HashMap<String, Vec<String>>> {
    let mongo_url = mongo_get_db_url();
    let providers_string_into_enum_hashmap: HashMap<String, ProviderKind> =
        ProviderKind::into_string_name_and_kind_hashmap();
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
