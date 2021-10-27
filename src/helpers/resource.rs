use crate::providers::provider_kind_enum::ProviderKind;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Resource {
    Local {
        path_to_provider_link_parts_folder: String,
        second_part_of_file_name: String
    },
    Mongodb {
        providers_string_into_enum_hashmap: HashMap<&'static str, ProviderKind>,
    },
    PostgreSql,
}
