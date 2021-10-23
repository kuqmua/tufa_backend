use crate::config_mods::get_config::enable_initialize_mongo_with_providers_link_parts_struct::EnableInitializeMongoWithProvidersLinkParts;
use crate::config_mods::get_config::mongo_url_parts_struct::MongoUrlParts;
use crate::config_mods::get_config::mongo_authorization_struct::MongoAuthorization;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MongoParams {
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
    pub mongo_url_parts: MongoUrlParts,
    pub mongo_authorization: MongoAuthorization,
}
