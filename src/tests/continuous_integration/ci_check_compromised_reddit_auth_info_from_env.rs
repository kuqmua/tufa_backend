use dotenv::dotenv;

use crate::constants::tests_constants::_USER_CREDENTIALS_DUMMY_HANDLE;

use crate::config_mods::env_var_enum::EnvVar;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_reddit_auth_info() {
    match dotenv() {
        Ok(_) => {
            match std::env::var(EnvVar::get_env_name(EnvVar::RedditUserAgent)) {
                Ok(reddit_user_agent) => {
                    assert!(
                        !(reddit_user_agent != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::get_env_name(EnvVar::RedditUserAgent),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_user_agent
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::get_env_name(EnvVar::RedditUserAgent),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::get_env_name(EnvVar::RedditClientId)) {
                Ok(reddit_client_id) => {
                    assert!(
                        !(reddit_client_id != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::get_env_name(EnvVar::RedditClientId),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_client_id
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::get_env_name(EnvVar::RedditClientId),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::get_env_name(EnvVar::RedditClientSecret)) {
                Ok(rediit_client_secret) => {
                    assert!(
                        !(rediit_client_secret != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::get_env_name(EnvVar::RedditClientSecret),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        rediit_client_secret
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::get_env_name(EnvVar::RedditClientSecret),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::get_env_name(EnvVar::RedditUsername)) {
                Ok(reddit_username) => {
                    assert!(
                        !(reddit_username != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::get_env_name(EnvVar::RedditUsername),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_username
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::get_env_name(EnvVar::RedditUsername),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::get_env_name(EnvVar::RedditPassword)) {
                Ok(reddit_password) => {
                    assert!(
                        !(reddit_password != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::get_env_name(EnvVar::RedditPassword),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        reddit_password
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::get_env_name(EnvVar::RedditPassword),
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
