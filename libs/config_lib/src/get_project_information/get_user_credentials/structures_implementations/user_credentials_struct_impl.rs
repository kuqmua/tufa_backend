use crate::get_project_information::project_constants::USER_CREDENTIALS_FILE_NAME;
use crate::get_project_information::get_user_credentials::structures_definitions::user_credentials_struct_def::UserCredentialsStruct;
use config::{Config, ConfigError, File};

impl UserCredentialsStruct {
    pub fn new(path_to_config: &str) -> Result<Self, ConfigError> {
        // maybe add different user logic later ?
        if !path_to_config.is_empty() {
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
                            UserCredentialsStruct::wrap_custom_config_checks(
                                user_credentials_handle,
                            )
                        }
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        } else {
            Err(ConfigError::Message("path_to_config.is_empty".to_string()))
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
