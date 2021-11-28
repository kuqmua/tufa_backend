use dotenv::dotenv;

use crate::constants::tests_constants::_USER_CREDENTIALS_DUMMY_HANDLE;

use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::get_env_name_trait::GetEnvName;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_github_auth_info() {
    match dotenv() {
        Ok(_) => {
            match std::env::var(EnvVar::GithubName.get_env_name()) {
                Ok(github_name) => {
                    assert!(
                        !(github_name != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::GithubName.get_env_name(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        github_name
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::GithubName.get_env_name(),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::GithubToken.get_env_name()) {
                Ok(github_token) => {
                    assert!(
                        !(github_token != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::GithubToken.get_env_name(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        github_token
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::GithubToken.get_env_name(),
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
