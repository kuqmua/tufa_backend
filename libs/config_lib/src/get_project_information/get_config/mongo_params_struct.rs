use crate::get_project_information::get_config::enable_initialize_mongo_with_providers_link_parts_struct::EnableInitializeMongoWithProvidersLinkParts;
use crate::get_project_information::get_config::enable_mongo_own_url_parts_struct::EnableMongoOwnUrlParts;
use crate::get_project_information::get_config::enable_mongo_cloud_url_parts_struct::EnableMongoCloudUrlParts;
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MongoParams {
    pub is_cloud: bool,
    pub providers_db_name_handle: String,
    pub providers_db_collection_handle_second_part: String,
    pub providers_db_collection_document_field_name_handle: String,
    //
    pub db_providers_logs_name_handle: String,
    pub db_providers_logs_collection_handle_second_part: String,
    pub db_providers_logs_collection_document_field_name_handle: String,
    //
    pub path_to_provider_link_parts_folder: String,
    pub log_file_extension: String,
    //
    pub enable_initialize_mongo_with_providers_link_parts:
        EnableInitializeMongoWithProvidersLinkParts,
    pub enable_mongo_own_url_parts: EnableMongoOwnUrlParts,
    pub enable_mongo_cloud_url_parts: EnableMongoCloudUrlParts,
}
