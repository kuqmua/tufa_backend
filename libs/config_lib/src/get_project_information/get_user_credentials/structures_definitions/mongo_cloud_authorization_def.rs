#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MongoCloudAuthorization {
    pub mongo_cloud_login: String,
    pub mongo_cloud_password: String,
    pub mongo_cloud_cluster_name: String,
    pub mongo_cloud_cluster_params: String,
}
