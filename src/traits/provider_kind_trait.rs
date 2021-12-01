use std::collections::HashMap;

use mongodb::bson::Document;

use crate::providers::provider_kind_enum::CleanLogsDirError;

pub trait ProviderKindTrait {
    fn is_link_limits_enabled(&self) -> bool;
    fn is_randomize_order_mongo_link_parts_enabled(&self) -> bool;
    fn get_check_link(&self) -> &'static str;
    fn get_init_local_data_file_path(&self) -> String;
    fn get_item_handle(&self) -> Option<&'static str>;
    fn get_links_limit_for_provider(&self) -> i64;
    fn get_mongo_doc_randomization_aggregation(&self) -> Option<Document>;
    fn get_mongo_log_collection_name(&self) -> String;
    fn get_path_to_logs_directory(&self) -> String;
    fn get_path_to_provider_log_file(&self) -> String;
    fn get_provider_links(&self, names_vector: Vec<String>) -> Vec<String>;
    fn is_cleaning_warning_logs_db_collections_in_mongo_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_directory_enable(&self) -> bool;
    fn is_enabled(&self) -> bool;
    fn is_mongo_initialization_enabled(&self) -> bool;
    fn is_prints_enabled(&self) -> bool;
    fn remove_logs_directory(&self) -> Result<(), CleanLogsDirError>;
    fn stringify(&self) -> &'static str;
    fn generate_hashmap_with_empty_string_vecs_for_enabled_providers() -> HashMap<Self, Vec<String>> where Self: Sized;
    fn get_enabled_providers_vec() -> Vec<Self> where Self: Sized;
    fn get_enabled_string_name_vec() -> Vec<String>;
    fn get_mongo_initialization_provider_kind_vec() -> Vec<Self> where Self: Sized;
}