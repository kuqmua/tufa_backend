use std::collections::HashMap;

use mongodb::bson::Document;

use crate::providers::provider_kind_enum::{CleanLogsDirError, RemoveDirError};

pub trait ProviderKindTrait {
    fn get_item_handle(&self) -> Option<&'static str>;
    fn get_mongo_doc_randomization_aggregation(&self) -> Option<Document>;
    fn get_mongo_log_collection_name(&self) -> String;
    fn get_path_to_logs_directory(&self) -> String;
    fn get_path_to_provider_log_file(&self) -> String;
    fn get_init_local_data_file_path(&self) -> String;
    fn remove_logs_directory(&self) -> Result<(), CleanLogsDirError>;
    fn stringify(&self) -> &'static str;
    fn generate_provider_links(&self, names_vector: Vec<String>) -> Vec<String>;
    fn generate_hashmap_with_empty_string_vecs_for_enabled_providers() -> HashMap<Self, Vec<String>>
    where
        Self: Sized;
    fn get_enabled_providers_vec() -> Vec<Self>
    where
        Self: Sized;
    fn get_enabled_string_name_vec() -> Vec<String>;
    fn get_mongo_initialization_provider_kind_vec() -> Vec<Self>
    where
        Self: Sized;
    fn get_mongo_initialization_string_name_vec() -> Vec<String>;
    fn into_string_name_and_kind_hashmap() -> HashMap<String, Self>
    where
        Self: Sized;
    fn into_string_name_and_kind_tuple_vec() -> Vec<(String, Self)>
    where
        Self: Sized;
    fn remove_existing_providers_logs_directories() -> Result<(), HashMap<Self, RemoveDirError>>
    where
        Self: Sized;
    fn remove_providers_logs_directories() -> Result<(), HashMap<Self, CleanLogsDirError>>
    where
        Self: Sized;
    fn get_db_tag(&self) -> String
    where
        Self: Sized;
}
