use crate::get_project_information::project_constants::GITHUB_NAME_ENV_NAME;
use crate::get_project_information::project_constants::GITHUB_TOKEN_ENV_NAME;
use crate::get_project_information::project_constants::REDDIT_USER_AGENT_ENV_NAME;
use crate::get_project_information::project_constants::REDDIT_CLIENT_ID_ENV_NAME;
use crate::get_project_information::project_constants::REDDIT_CLIENT_SECRET_ENV_NAME;
use crate::get_project_information::project_constants::REDDIT_USERNAME_ENV_NAME;
use crate::get_project_information::project_constants::REDDIT_PASSWORD_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_OWN_LOGIN_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_OWN_PASSWORD_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_OWN_IP_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_OWN_PORT_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_CLOUD_LOGIN_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_CLOUD_PASSWORD_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_CLOUD_CLUSTER_NAME_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_CLOUD_CLUSTER_PARAMS_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_OWN_LOGIN_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_OWN_PASSWORD_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_OWN_IP_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_OWN_DB_ENV_NAME;
use crate::get_project_information::get_user_credentials::structures_definitions::user_credentials_struct_def::UserCredentialsStruct;
use config::{ConfigError};

use crate::get_project_information::get_user_credentials::structures_definitions::github_authorization_def::GithubAuthorization;
use crate::get_project_information::get_user_credentials::structures_definitions::reddit_authorization_def::RedditAuthorization;
use crate::get_project_information::get_user_credentials::structures_definitions::mongo_own_authorization_def::MongoOwnAuthorization;
use crate::get_project_information::get_user_credentials::structures_definitions::postgres_own_authorization_def::PostgresOwnAuthorization;
use crate::get_project_information::get_user_credentials::structures_definitions::mongo_cloud_authorization_def::MongoCloudAuthorization;

use dotenv::dotenv;

pub struct SomethingOne {
    pub github_authorization: SomethingTwo,
}

pub struct SomethingTwo {
    pub github_authorization: String,
}

impl UserCredentialsStruct {
    pub fn new() -> Result<Self, ConfigError> {
        //todo remove configError to just std::error
        let dotenv_result = dotenv();
        match dotenv_result {
            Ok(_) => {
                let mut handle_user_credentials: UserCredentialsStruct = UserCredentialsStruct {
                    github_authorization: GithubAuthorization {
                        github_name: "".to_string(),
                        github_token: "".to_string(),
                    },
                    reddit_authorization: RedditAuthorization {
                        reddit_user_agent: "".to_string(),
                        reddit_client_id: "".to_string(),
                        reddit_client_secret: "".to_string(),
                        reddit_username: "".to_string(),
                        reddit_password: "".to_string(),
                    },
                    mongo_own_authorization: MongoOwnAuthorization {
                        mongo_own_login: "".to_string(),
                        mongo_own_password: "".to_string(),
                        mongo_own_ip: "".to_string(),
                        mongo_own_port: "".to_string(),
                    },
                    postgres_own_authorization: PostgresOwnAuthorization {
                        postgres_own_login: "".to_string(),
                        postgres_own_password: "".to_string(),
                        postgres_own_ip: "".to_string(),
                        postgres_own_db: "".to_string(),
                    },
                    mongo_cloud_authorization: MongoCloudAuthorization {
                        mongo_cloud_login: "".to_string(),
                        mongo_cloud_password: "".to_string(),
                        mongo_cloud_cluster_name: "".to_string(),
                        mongo_cloud_cluster_params: "".to_string(),
                    },
                };
                //todo: add errors
                match std::env::var(GITHUB_NAME_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.github_authorization.github_name = handle;
                    }
                    Err(e) => {

                        return Err(ConfigError::Message(format!("std::env::var(GITHUB_NAME_ENV_NAME({})) failed for console and .env file, error", GITHUB_NAME_ENV_NAME)))
                    }
                }
                match std::env::var(GITHUB_TOKEN_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.github_authorization.github_token = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(GITHUB_TOKEN_ENV_NAME({})) failed for console and .env file, error", GITHUB_TOKEN_ENV_NAME)))
                    }
                }
                match std::env::var(REDDIT_USER_AGENT_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.reddit_authorization.reddit_user_agent = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(REDDIT_USER_AGENT_ENV_NAME({})) failed for console and .env file, error", REDDIT_USER_AGENT_ENV_NAME)))
                    }
                }
                match std::env::var(REDDIT_CLIENT_ID_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.reddit_authorization.reddit_client_id = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(REDDIT_CLIENT_ID_ENV_NAME({})) failed for console and .env file, error", REDDIT_CLIENT_ID_ENV_NAME)))
                    }
                }
                match std::env::var(REDDIT_CLIENT_SECRET_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.reddit_authorization.reddit_client_secret = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(REDDIT_CLIENT_SECRET_ENV_NAME({})) failed for console and .env file, error", REDDIT_CLIENT_SECRET_ENV_NAME)))
                    }
                }
                match std::env::var(REDDIT_USERNAME_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.reddit_authorization.reddit_username = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(REDDIT_USERNAME_ENV_NAME({})) failed for console and .env file, error", REDDIT_USERNAME_ENV_NAME)))
                    }
                }
                match std::env::var(REDDIT_PASSWORD_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.reddit_authorization.reddit_password = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(REDDIT_PASSWORD_ENV_NAME({})) failed for console and .env file, error", REDDIT_PASSWORD_ENV_NAME)))
                    }
                }
                match std::env::var(MONGO_OWN_LOGIN_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.mongo_own_authorization.mongo_own_login = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(MONGO_OWN_LOGIN_ENV_NAME({})) failed for console and .env file, error", MONGO_OWN_LOGIN_ENV_NAME)))
                    }
                }
                match std::env::var(MONGO_OWN_PASSWORD_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.mongo_own_authorization.mongo_own_password = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(MONGO_OWN_PASSWORD_ENV_NAME({})) failed for console and .env file, error", MONGO_OWN_PASSWORD_ENV_NAME)))
                    }
                }
                match std::env::var(MONGO_OWN_IP_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.mongo_own_authorization.mongo_own_ip = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(MONGO_OWN_IP_ENV_NAME({})) failed for console and .env file, error", MONGO_OWN_IP_ENV_NAME)))
                    }
                }
                match std::env::var(MONGO_OWN_PORT_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.mongo_own_authorization.mongo_own_port = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(MONGO_OWN_PORT_ENV_NAME({})) failed for console and .env file, error", MONGO_OWN_PORT_ENV_NAME)))
                    }
                }
                match std::env::var(MONGO_CLOUD_LOGIN_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.mongo_cloud_authorization.mongo_cloud_login = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(MONGO_CLOUD_LOGIN_ENV_NAME({})) failed for console and .env file, error", MONGO_CLOUD_LOGIN_ENV_NAME)))
                    }
                }
                match std::env::var(MONGO_CLOUD_PASSWORD_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.mongo_cloud_authorization.mongo_cloud_password = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(MONGO_CLOUD_PASSWORD_ENV_NAME({})) failed for console and .env file, error", MONGO_CLOUD_PASSWORD_ENV_NAME)))
                    }
                }
                match std::env::var(MONGO_CLOUD_CLUSTER_NAME_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.mongo_cloud_authorization.mongo_cloud_cluster_name = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(MONGO_CLOUD_CLUSTER_NAME_ENV_NAME({})) failed for console and .env file, error", MONGO_CLOUD_CLUSTER_NAME_ENV_NAME)))
                    }
                }
                match std::env::var(MONGO_CLOUD_CLUSTER_PARAMS_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.mongo_cloud_authorization.mongo_cloud_cluster_params = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(MONGO_CLOUD_CLUSTER_PARAMS_ENV_NAME({})) failed for console and .env file, error", MONGO_CLOUD_CLUSTER_PARAMS_ENV_NAME)))
                    }
                }
                match std::env::var(POSTGRES_OWN_LOGIN_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.postgres_own_authorization.postgres_own_login = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(POSTGRES_OWN_LOGIN_ENV_NAME({})) failed for console and .env file, error", POSTGRES_OWN_LOGIN_ENV_NAME)))
                    }
                }
                match std::env::var(POSTGRES_OWN_PASSWORD_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.postgres_own_authorization.postgres_own_password = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(POSTGRES_OWN_PASSWORD_ENV_NAME({})) failed for console and .env file, error", POSTGRES_OWN_PASSWORD_ENV_NAME)))
                    }
                }
                match std::env::var(POSTGRES_OWN_IP_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.postgres_own_authorization.postgres_own_ip = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(POSTGRES_OWN_IP_ENV_NAME({})) failed for console and .env file, error", POSTGRES_OWN_IP_ENV_NAME)))
                    }
                }
                match std::env::var(POSTGRES_OWN_DB_ENV_NAME) {
                    Ok(handle) => {
                        handle_user_credentials.postgres_own_authorization.postgres_own_db = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!("std::env::var(POSTGRES_OWN_DB_ENV_NAME({})) failed for console and .env file, error", POSTGRES_OWN_DB_ENV_NAME)))
                    }
                }
                match UserCredentialsStruct::wrap_custom_config_checks(handle_user_credentials) {
                    Ok(handle) => {
                        return Ok(handle);
                    }
                    Err(e) => {
                        return Err(ConfigError::Message(format!(
                            "wrap_custom_config_checks error {:#?}",
                            e
                        )))
                    }
                }
            }
            Err(e) => {
                //todo: console
                return Err(ConfigError::Message("dotenv error".to_string()));
            }
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
