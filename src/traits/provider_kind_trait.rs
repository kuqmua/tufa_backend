use std::collections::HashMap;

use mongodb::bson::Document;

use crate::providers::provider_kind_enum::{CleanLogsDirError, RemoveDirError};

pub trait ProviderKindTrait {
    fn is_mongo_initialization_enabled(&self) -> bool;
    fn is_mongo_write_error_logs_enabled(&self) -> bool;
    fn is_mongo_cleaning_warning_logs_db_enabled(&self) -> bool;
    fn is_mongo_cleaning_warning_logs_db_collections_enabled(&self) -> bool;
    fn is_mongo_link_parts_randomize_order_enabled(&self) -> bool;

    fn is_postgres_initialization_enabled(&self) -> bool;

    fn is_write_error_logs_in_local_folder_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_directory_enabled(&self) -> bool;

    fn get_check_link(&self) -> &'static str;

    fn is_enabled(&self) -> bool;
    fn is_prints_enabled(&self) -> bool;
    fn is_warning_high_prints_enabled(&self) -> bool;
    fn is_warning_low_prints_enabled(&self) -> bool;
    fn is_success_prints_enabled(&self) -> bool;
    fn is_partial_success_prints_enabled(&self) -> bool;
    fn is_error_prints_enabled(&self) -> bool;
    fn is_time_measurement_prints_enabled(&self) -> bool;
    fn is_info_prints_enabled(&self) -> bool;

    fn is_links_limit_enabled(&self) -> bool;
    fn get_links_limit(&self) -> i64;
    ///
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
}
