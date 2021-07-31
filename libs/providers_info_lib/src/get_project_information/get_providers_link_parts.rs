use crate::get_project_information::get_providers_link_parts_from_mongo::get_providers_link_parts_from_mongo;
use config_lib::get_project_information::get_config::get_config_information::CONFIG;
use config_lib::get_project_information::project_constants::get_config_provider_string_to_enum_struct;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use std::collections::HashMap;

use crate::get_project_information::get_providers_json_local_data::get_providers_json_local_data;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn get_providers_link_parts(resource: &Resource) -> HashMap<String, Vec<String>> {
    let vec_of_link_parts_hashmap: HashMap<String, Vec<String>>;
    let config_provider_string_to_enum_struct_hashmap = get_config_provider_string_to_enum_struct();
    let mut providers_string_into_enum_hashmap: HashMap<String, ProviderKind> =
        HashMap::with_capacity(CONFIG.params.vec_of_provider_names.len());
    for provider_name in &CONFIG.params.vec_of_provider_names {
        match config_provider_string_to_enum_struct_hashmap.get(provider_name) {
            Some(provider_kind_handle) => {
                providers_string_into_enum_hashmap
                    .insert(provider_name.to_string(), provider_kind_handle.clone());
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
            providers_string_into_enum_hashmap,
        } => {
            vec_of_link_parts_hashmap = get_providers_link_parts_from_mongo(
                mongo_url.to_string(),
                db_name_handle.to_string(),
                db_collection_handle_second_part.to_string(),
                db_collection_document_field_name_handle.to_string(),
                providers_string_into_enum_hashmap.clone(),
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
        providers_string_into_enum_hashmap: HashMap<String, ProviderKind>,
    },
    PostgreSql,
}
