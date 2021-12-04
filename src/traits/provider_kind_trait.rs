use std::collections::HashMap;

use mongodb::bson::Document;

use crate::providers::provider_kind_enum::{CleanLogsDirError, RemoveDirError};

use crate::providers::get_providers_json_local_data_processed_error::GetProvidersJsonLocalDataProcessedError;

pub trait ProviderKindTrait {
    fn is_mongo_initialization_enabled(&self) -> bool;
    fn is_enabled(&self) -> bool;
    fn is_prints_enabled(&self) -> bool;
    fn is_warning_high_prints_enabled(&self) -> bool;
    fn is_warning_low_prints_enabled(&self) -> bool;
    fn is_error_prints_enabled(&self) -> bool;
    fn is_success_prints_enabled(&self) -> bool;
    fn is_partial_success_prints_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_directory_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_db_in_mongo_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_db_collections_in_mongo_enabled(&self) -> bool;
    fn is_time_measurement_enabled(&self) -> bool;
    fn is_info_prints_enabled(&self) -> bool;
    fn is_link_limits_enabled(&self) -> bool;
    fn is_randomize_order_mongo_link_parts_enabled(&self) -> bool;
    fn is_cleaning_warning_logs_directory_enable(&self) -> bool;

    fn get_check_link(&self) -> &'static str;
    fn get_item_handle(&self) -> Option<&'static str>;
    fn get_links_limit_for_provider(&self) -> i64;
    fn get_mongo_doc_randomization_aggregation(&self) -> Option<Document>;
    fn get_mongo_log_collection_name(&self) -> String;
    fn get_path_to_logs_directory(&self) -> String;
    fn get_path_to_provider_log_file(&self) -> String;
    fn get_init_local_data_file_path(&self) -> String;
    fn remove_logs_directory(&self) -> Result<(), CleanLogsDirError>;
    fn stringify(&self) -> &'static str;
    fn get_provider_links(&self, names_vector: Vec<String>) -> Vec<String>;
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
    fn get_providers_json_local_data_processed() -> (
        HashMap<Self, Vec<String>>,
        HashMap<Self, GetProvidersJsonLocalDataProcessedError>,
    )
    where
        Self: Sized;
    fn get_providers_json_local_data_unprocessed(
    ) -> HashMap<Self, Result<Result<Vec<String>, serde_json::Error>, std::io::Error>>
    where
        Self: Sized;
    fn into_string_name_and_kind_hashmap() -> HashMap<String, Self>
    where
        Self: Sized;
    fn into_string_name_and_kind_tuple_vec() -> Vec<(String, Self)>
    where
        Self: Sized;
    fn into_vec() -> Vec<Self>
    where
        Self: Sized;
    fn remove_existing_providers_logs_directories() -> Result<(), HashMap<Self, RemoveDirError>>
    where
        Self: Sized;
    fn remove_providers_logs_directories() -> Result<(), HashMap<Self, CleanLogsDirError>>
    where
        Self: Sized;
}
