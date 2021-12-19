use dotenv::dotenv;

use crate::constants::tests_constants::_USER_CREDENTIALS_DUMMY_HANDLE;

use crate::config_mods::env_var_enum::EnvVar;
use crate::traits::enum_extention::EnumExtenstion;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_reddit_auth_info() {
    match dotenv() {
        Ok(_) => {
            match std::env::var(EnvVar::RedditUserAgent.to_upper_snake_case()) {
                Ok(reddit_user_agent) => {
                    assert!(
                        !(reddit_user_agent != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::RedditUserAgent.to_upper_snake_case(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_user_agent
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::RedditUserAgent.to_upper_snake_case(),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::RedditClientId.to_upper_snake_case()) {
                Ok(reddit_client_id) => {
                    assert!(
                        !(reddit_client_id != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::RedditClientId.to_upper_snake_case(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_client_id
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::RedditClientId.to_upper_snake_case(),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::RedditClientSecret.to_upper_snake_case()) {
                Ok(rediit_client_secret) => {
                    assert!(
                        !(rediit_client_secret != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::RedditClientSecret.to_upper_snake_case(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        rediit_client_secret
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::RedditClientSecret.to_upper_snake_case(),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::RedditUsername.to_upper_snake_case()) {
                Ok(reddit_username) => {
                    assert!(
                        !(reddit_username != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::RedditUsername.to_upper_snake_case(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_username
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::RedditUsername.to_upper_snake_case(),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::RedditPassword.to_upper_snake_case()) {
                Ok(reddit_password) => {
                    assert!(
                        !(reddit_password != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::RedditPassword.to_upper_snake_case(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_password
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::RedditPassword.to_upper_snake_case(),
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
