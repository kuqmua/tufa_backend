use crate::constants::env_var_names_constants::REDDIT_CLIENT_ID_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_CLIENT_SECRET_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_PASSWORD_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_USERNAME_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_USER_AGENT_ENV_NAME;

use crate::constants::tests_constants::_USER_CREDENTIALS_DUMMY_HANDLE;

use dotenv::dotenv;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_reddit_auth_info() {
    match dotenv() {
        Ok(_) => {
            match std::env::var(REDDIT_USER_AGENT_ENV_NAME) {
                Ok(reddit_user_agent) => {
                    assert!(
                        !(reddit_user_agent != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        REDDIT_USER_AGENT_ENV_NAME,
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_user_agent
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        REDDIT_USER_AGENT_ENV_NAME, e
                    );
                }
            }
            match std::env::var(REDDIT_CLIENT_ID_ENV_NAME) {
                Ok(reddit_client_id) => {
                    assert!(
                        !(reddit_client_id != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        REDDIT_CLIENT_ID_ENV_NAME,
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_client_id
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        REDDIT_CLIENT_ID_ENV_NAME, e
                    );
                }
            }
            match std::env::var(REDDIT_CLIENT_SECRET_ENV_NAME) {
                Ok(rediit_client_secret) => {
                    assert!(
                        !(rediit_client_secret != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        REDDIT_CLIENT_SECRET_ENV_NAME,
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        rediit_client_secret
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        REDDIT_CLIENT_SECRET_ENV_NAME, e
                    );
                }
            }
            match std::env::var(REDDIT_USERNAME_ENV_NAME) {
                Ok(reddit_username) => {
                    assert!(
                        !(reddit_username != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        REDDIT_USERNAME_ENV_NAME,
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_username
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        REDDIT_USERNAME_ENV_NAME, e
                    );
                }
            }
            match std::env::var(REDDIT_PASSWORD_ENV_NAME) {
                Ok(reddit_password) => {
                    assert!(
                        !(reddit_password != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        REDDIT_PASSWORD_ENV_NAME,
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_password
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        REDDIT_PASSWORD_ENV_NAME, e
                    );
                }
            }
        }
        Err(e) => {
            panic!("dotenv() failed error: {:?}", e);
        }
    }
}
