use dotenv::dotenv;

use crate::constants::tests_constants::_USER_CREDENTIALS_DUMMY_HANDLE;

use crate::config_mods::env_var_enum::EnvVar;
use crate::traits::enum_extention::EnumExtenstion;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_github_auth_info() {
    match dotenv() {
        Ok(_) => {
            match std::env::var(EnvVar::GithubName.to_upper_snake_case()) {
                Ok(github_name) => {
                    assert!(
                        !(github_name != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::GithubName.to_upper_snake_case(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        github_name
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::GithubName.to_upper_snake_case(),
                        e
                    );
                }
            }
            match std::env::var(EnvVar::GithubToken.to_upper_snake_case()) {
                Ok(github_token) => {
                    assert!(
                        !(github_token != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        EnvVar::GithubToken.to_upper_snake_case(),
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        github_token
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        EnvVar::GithubToken.to_upper_snake_case(),
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
