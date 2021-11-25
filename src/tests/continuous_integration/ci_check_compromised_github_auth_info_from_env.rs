use dotenv::dotenv;

use crate::constants::tests_constants::_USER_CREDENTIALS_DUMMY_HANDLE;

use crate::config_mods::env_var_enum::EnvVar;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_github_auth_info() {
    match dotenv() {
        Ok(_) => {
            match std::env::var(EnvVar::get_env_name(EnvVar::GithubName)) {
                Ok(github_name) => {
                    assert!(
                        !(github_name != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::get_env_name(EnvVar::GithubName),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        github_name
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::get_env_name(EnvVar::GithubName), e
                    );
                }
            }
            match std::env::var(EnvVar::get_env_name(EnvVar::GithubToken)) {
                Ok(github_token) => {
                    assert!(
                        !(github_token != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::get_env_name(EnvVar::GithubToken),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        github_token
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::get_env_name(EnvVar::GithubToken), e
                    );
                }
            }
        }
        Err(e) => {
            panic!("dotenv() failed error: {:?}", e);
        }
    }
}
