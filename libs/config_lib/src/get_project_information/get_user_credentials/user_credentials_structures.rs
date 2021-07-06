use crate::get_project_information::project_constants::USER_CREDENTIALS_FILE_NAME;
use config::{Config, ConfigError, File};
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)] //Default,
pub struct UserCredentialsStruct {
    pub github_authorization: GithubAuthorization,
    pub reddit_authorization: RedditAuthorization,
    pub mongo_own_authorization: MongoOwnAuthorization,
    pub mongo_cloud_authorization: MongoCloudAuthorization,
}

impl UserCredentialsStruct {
    pub fn new(path_to_config: &str) -> Result<Self, ConfigError> {
        // maybe add different user logic later ?
        let mut config = Config::new();
        config.merge(File::with_name(&format!(
            "{}{}",
            path_to_config, USER_CREDENTIALS_FILE_NAME
        )))?;
        config.try_into()
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GithubAuthorization {
    pub github_name: String,
    pub github_token: String,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditAuthorization {
    pub reddit_user_agent: String,
    pub reddit_client_id: String,
    pub reddit_client_secret: String,
    pub reddit_username: String,
    pub reddit_password: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MongoOwnAuthorization {
    pub mongo_own_login: String,
    pub mongo_own_password: String,
    pub mongo_own_ip: String,
    pub mongo_own_port: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MongoCloudAuthorization {
    pub mongo_cloud_login: String,
    pub mongo_cloud_password: String,
    pub mongo_cloud_cluster_name: String,
    pub mongo_cloud_cluster_params: String,
}
