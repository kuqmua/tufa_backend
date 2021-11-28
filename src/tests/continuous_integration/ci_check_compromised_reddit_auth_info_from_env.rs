use dotenv::dotenv;

use crate::constants::tests_constants::_USER_CREDENTIALS_DUMMY_HANDLE;

use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::get_env_name_trait::GetEnvName;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_reddit_auth_info() {
    match dotenv() {
        Ok(_) => {
            match std::env::var(EnvVar::RedditUserAgent.get_env_name()) {
                Ok(reddit_user_agent) => {
                    assert!(
                        !(reddit_user_agent != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::RedditUserAgent.get_env_name(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_user_agent
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::RedditUserAgent.get_env_name(),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::RedditClientId.get_env_name()) {
                Ok(reddit_client_id) => {
                    assert!(
                        !(reddit_client_id != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::RedditClientId.get_env_name(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_client_id
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::RedditClientId.get_env_name(),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::RedditClientSecret.get_env_name()) {
                Ok(rediit_client_secret) => {
                    assert!(
                        !(rediit_client_secret != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::RedditClientSecret.get_env_name(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        rediit_client_secret
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::RedditClientSecret.get_env_name(),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::RedditUsername.get_env_name()) {
                Ok(reddit_username) => {
                    assert!(
                        !(reddit_username != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::RedditUsername.get_env_name(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_username
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::RedditUsername.get_env_name(),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::RedditPassword.get_env_name()) {
                Ok(reddit_password) => {
                    assert!(
                        !(reddit_password != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::RedditPassword.get_env_name(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_password
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::RedditPassword.get_env_name(),
                        e
                    );
                }
            }
        }
        Err(e) => {
            panic!("dotenv() failed error: {:?}", e);
        }
    }
}
