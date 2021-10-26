use crate::providers::providers_info::get_providers_json_local_data::get_providers_json_local_data;
use crate::providers::providers_info::get_providers_link_parts_from_mongo::get_providers_link_parts_from_mongo;

use std::collections::HashMap;

use crate::helpers::resource::Resource;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_as_hashmap(
    resource: &Resource,
) -> HashMap<&'static str, Vec<String>> {
    let vec_of_link_parts_hashmap: HashMap<&'static str, Vec<String>>;
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
            db_name_handle,
            db_collection_document_field_name_handle,
            providers_string_into_enum_hashmap,
        } => {
            vec_of_link_parts_hashmap = get_providers_link_parts_from_mongo(
                db_name_handle.to_string(),
                db_collection_document_field_name_handle.to_string(),
                providers_string_into_enum_hashmap.clone(),
            )
            .await;
        }
        Resource::PostgreSql => {
            todo!()
        }
    }
    vec_of_link_parts_hashmap
}

////////////////////////
// #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
// pub async fn get_providers_link_parts_as_hashmap(resource: &Resource) -> HashMap<String, Vec<String>> {
//     let vec_of_link_parts_hashmap: HashMap<String, Vec<String>>;
//     match resource {
//         Resource::Local {
//             path_to_provider_link_parts_folder,
//             vec_of_provider_names,
//             second_part_of_file_name,
//             file_extension,
//         } => {
//             vec_of_link_parts_hashmap = get_providers_json_local_data(
//                 path_to_provider_link_parts_folder,
//                 vec_of_provider_names.to_vec(),
//                 second_part_of_file_name,
//                 file_extension,
//             );
//         }
//         Resource::Mongodb {
//             mongo_url,
//             db_name_handle,
//             db_collection_handle_second_part,
//             db_collection_document_field_name_handle,
//             providers_string_into_enum_hashmap,
//         } => {
//             vec_of_link_parts_hashmap = get_providers_link_parts_from_mongo(
//                 mongo_url.to_string(),
//                 db_name_handle.to_string(),
//                 db_collection_handle_second_part.to_string(),
//                 db_collection_document_field_name_handle.to_string(),
//                 providers_string_into_enum_hashmap.clone(),
//             )
//             .await;
//         }
//         Resource::PostgreSql => {
//             todo!()
//         }
//     }
//     vec_of_link_parts_hashmap
// }
