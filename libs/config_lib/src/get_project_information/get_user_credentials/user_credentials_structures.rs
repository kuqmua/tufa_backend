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
        //todo: check path_to_config is empty
        // maybe add different user logic later ?
        let mut config = Config::new();
        let config_merge_result = config.merge(File::with_name(&format!(
            "{}{}",
            path_to_config, USER_CREDENTIALS_FILE_NAME
        )));
        match config_merge_result {
            Ok(_) => {
                // config.try_into();
                let config_result: Result<Self, ConfigError> = config.try_into();
                match config_result {
                    Ok(user_credentials_handle) => {
                        UserCredentialsStruct::wrap_custom_config_checks(user_credentials_handle)
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn wrap_custom_config_checks(
        user_credentials_handle: UserCredentialsStruct,
    ) -> Result<Self, ConfigError> {
        if !user_credentials_handle
            .github_authorization
            .github_name
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> =
                Err(ConfigError::Message("github_name is not valid".to_string()));
            drop(error);
        }
        if !user_credentials_handle
            .github_authorization
            .github_token
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "github_token is not valid".to_string(),
            ));
            drop(error);
        }
        //
        if !user_credentials_handle
            .reddit_authorization
            .reddit_user_agent
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "reddit_user_agent is not valid".to_string(),
            ));
            drop(error);
        }
        if !user_credentials_handle
            .reddit_authorization
            .reddit_client_id
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "reddit_client_id is not valid".to_string(),
            ));
            drop(error);
        }
        if !user_credentials_handle
            .reddit_authorization
            .reddit_client_secret
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "reddit_client_secret is not valid".to_string(),
            ));
            drop(error);
        }
        if !user_credentials_handle
            .reddit_authorization
            .reddit_username
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "reddit_username is not valid".to_string(),
            ));
            drop(error);
        }
        if !user_credentials_handle
            .reddit_authorization
            .reddit_password
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "reddit_password is not valid".to_string(),
            ));
            drop(error);
        }
        /////////
        if !user_credentials_handle
            .mongo_own_authorization
            .mongo_own_login
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_own_login is not valid".to_string(),
            ));
            drop(error);
        }
        if !user_credentials_handle
            .mongo_own_authorization
            .mongo_own_password
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_own_password is not valid".to_string(),
            ));
            drop(error);
        }
        if !user_credentials_handle
            .mongo_own_authorization
            .mongo_own_ip
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_own_ip is not valid".to_string(),
            ));
            drop(error);
        }
        if !user_credentials_handle
            .mongo_own_authorization
            .mongo_own_port
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_own_port is not valid".to_string(),
            ));
            drop(error);
        }
        /////////
        if !user_credentials_handle
            .mongo_cloud_authorization
            .mongo_cloud_login
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_cloud_login is not valid".to_string(),
            ));
            drop(error);
        }
        if !user_credentials_handle
            .mongo_cloud_authorization
            .mongo_cloud_password
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_cloud_password is not valid".to_string(),
            ));
            drop(error);
        }
        if !user_credentials_handle
            .mongo_cloud_authorization
            .mongo_cloud_cluster_name
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_cloud_cluster_name is not valid".to_string(),
            ));
            drop(error);
        }
        if !user_credentials_handle
            .mongo_cloud_authorization
            .mongo_cloud_cluster_params
            .is_empty()
        {
            let error: Result<UserCredentialsStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_cloud_cluster_params is not valid".to_string(),
            ));
            drop(error);
        }
        Ok(user_credentials_handle)
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
