#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableMongoCloudUrlParts {
    pub mongo_cloud_first_handle_url_part: String,
    pub mongo_cloud_second_handle_url_part: String,
    pub mongo_cloud_third_handle_url_part: String,
    pub mongo_cloud_fourth_handle_url_part: String,
}
