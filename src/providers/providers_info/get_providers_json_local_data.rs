use std::collections::HashMap;
use std::fs;

use crate::config_mods::config::CONFIG;
use crate::providers::provider_kind_enum::ProviderKind;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn get_providers_json_local_data() -> HashMap<&'static str, Vec<String>> {
    let mut vec_of_link_parts_hashmap: HashMap<&'static str, Vec<String>> = HashMap::new();
    //todo: do it async in parallel
    for provider_name in ProviderKind::get_enabled_string_name_vec() {
        let result_of_reading_to_string = fs::read_to_string(&format!(
            "{}{}{}{}",
            CONFIG.mongo_params.path_to_provider_link_parts_folder,
            provider_name,
            CONFIG
                .mongo_params
                .providers_db_collection_handle_second_part,
            CONFIG.mongo_params.log_file_extension
        ));
        match result_of_reading_to_string {
            Ok(file_content) => {
                let file_content_from_str_result: Result<
                    ProviderLinkPartStruct,
                    serde_json::Error,
                > = serde_json::from_str(&file_content);
                match file_content_from_str_result {
                    Ok(file_content_as_struct) => {
                        let mut vec_of_link_parts: Vec<String> =
                            Vec::with_capacity(file_content_as_struct.data.len());
                        for link_part in file_content_as_struct.data {
                            vec_of_link_parts.push(link_part)
                        }
                        vec_of_link_parts_hashmap.insert(provider_name, vec_of_link_parts);
                    }
                    Err(e) => println!("file_content_from_str_result error {:#?}", e),
                }
            }
            Err(e) => {
                println!(
                    "cannot read_to_string from file {}{}{}{}, reason: {}",
                    CONFIG.mongo_params.path_to_provider_link_parts_folder,
                    provider_name,
                    CONFIG
                        .mongo_params
                        .providers_db_collection_handle_second_part,
                    CONFIG.mongo_params.log_file_extension,
                    e
                )
            }
        }
    }
    // println!("vec_of_link_parts_hashmap {:#?}", vec_of_link_parts_hashmap)
    vec_of_link_parts_hashmap
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ProviderLinkPartStruct {
    pub data: Vec<String>,
}
