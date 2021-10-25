use std::collections::HashMap;
use crate::providers::provider_kind_enum::ProviderKind;

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