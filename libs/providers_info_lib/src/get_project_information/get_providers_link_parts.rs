use mongo_integration::mongo_get_provider_link_parts_as_bson_string::mongo_get_provider_link_parts_as_bson_string;
use std::collections::HashMap;

pub fn get_providers_link_parts(
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_handle_second_part: &str,
    db_collection_document_field_name_handle: &str,
    vec_of_provider_names: Vec<String>,
) -> HashMap<String, Vec<String>> {
    let mut vec_of_link_parts_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    for provider_name in vec_of_provider_names {
        let result_getting_provider_link_parts = mongo_get_provider_link_parts_as_bson_string(
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
    vec_of_link_parts_hashmap
}
