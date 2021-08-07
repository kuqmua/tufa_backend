#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MongoParams {
    pub is_cloud: bool,
    pub db_name_handle: String,
    pub db_collection_handle_second_part: String,
    pub db_collection_document_field_name_handle: String,
    pub path_to_provider_link_parts_folder: String,
    pub file_extension: String,
    //
    pub mongo_own_first_handle_url_part: String,
    pub mongo_own_second_handle_url_part: String,
    pub mongo_own_third_handle_url_part: String,
    pub mongo_own_fourth_handle_url_part: String,
    //
    pub mongo_cloud_first_handle_url_part: String,
    pub mongo_cloud_second_handle_url_part: String,
    pub mongo_cloud_third_handle_url_part: String,
    pub mongo_cloud_fourth_handle_url_part: String,
}
