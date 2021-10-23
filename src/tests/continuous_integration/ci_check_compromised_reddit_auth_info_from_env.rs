use crate::constants::env_var_names_constants::REDDIT_USER_AGENT_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_CLIENT_ID_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_CLIENT_SECRET_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_USERNAME_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_PASSWORD_ENV_NAME;

use crate::get_project_information::get_config::get_lazy_config_information::CONFIG;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_reddit_auth_info() {
    match std::env::var(REDDIT_USER_AGENT_ENV_NAME) {
        Ok(reddit_user_agent) => {
            if reddit_user_agent != CONFIG.params.user_credentials_dummy_handle {
                panic!(
                    "{} != {}, found {}", REDDIT_USER_AGENT_ENV_NAME,
                    CONFIG.params.user_credentials_dummy_handle,
                    reddit_user_agent
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", REDDIT_USER_AGENT_ENV_NAME, e);
        }   
    }
    match std::env::var(REDDIT_CLIENT_ID_ENV_NAME) {
        Ok(reddit_client_id) => {
            if reddit_client_id != CONFIG.params.user_credentials_dummy_handle {
                panic!(
                    "{} != {}, found {}", REDDIT_CLIENT_ID_ENV_NAME,
                    CONFIG.params.user_credentials_dummy_handle,
                    reddit_client_id
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", REDDIT_CLIENT_ID_ENV_NAME, e);
        }   
    }
    match std::env::var(REDDIT_CLIENT_SECRET_ENV_NAME) {
        Ok(rediit_client_secret) => {
            if rediit_client_secret != CONFIG.params.user_credentials_dummy_handle {
                panic!(
                    "{} != {}, found {}", REDDIT_CLIENT_SECRET_ENV_NAME,
                    CONFIG.params.user_credentials_dummy_handle,
                    rediit_client_secret
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", REDDIT_CLIENT_SECRET_ENV_NAME, e);
        }   
    }
    match std::env::var(REDDIT_USERNAME_ENV_NAME) {
        Ok(reddit_username) => {
            if reddit_username != CONFIG.params.user_credentials_dummy_handle {
                panic!(
                    "{} != {}, found {}", REDDIT_USERNAME_ENV_NAME,
                    CONFIG.params.user_credentials_dummy_handle,
                    reddit_username
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", REDDIT_USERNAME_ENV_NAME, e);
        }   
    }
    match std::env::var(REDDIT_PASSWORD_ENV_NAME) {
        Ok(reddit_password) => {
            if reddit_password != CONFIG.params.user_credentials_dummy_handle {
                panic!(
                    "{} != {}, found {}", REDDIT_PASSWORD_ENV_NAME,
                    CONFIG.params.user_credentials_dummy_handle,
                    reddit_password
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", REDDIT_PASSWORD_ENV_NAME, e);
        }   
    }
}
