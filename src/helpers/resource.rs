use crate::providers::provider_kind_enum::ProviderKind;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Resource {
    Local {
        path_to_provider_link_parts_folder: String,
        vec_of_provider_names: Vec<String>,
        second_part_of_file_name: String,
        file_extension: String,
    },
    Mongodb {
        db_name_handle: String,
        db_collection_document_field_name_handle: String,
        providers_string_into_enum_hashmap: HashMap<String, ProviderKind>,
    },
    PostgreSql,
}
