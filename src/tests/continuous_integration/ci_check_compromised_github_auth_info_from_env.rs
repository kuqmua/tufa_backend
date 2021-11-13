use crate::constants::env_var_names_constants::GITHUB_NAME_ENV_NAME;
use crate::constants::env_var_names_constants::GITHUB_TOKEN_ENV_NAME;
use crate::constants::tests_constants::_USER_CREDENTIALS_DUMMY_HANDLE;

use dotenv::dotenv;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_github_auth_info() {
    match dotenv() {
        Ok(_) => {
            match std::env::var(GITHUB_NAME_ENV_NAME) {
                Ok(github_name) => {
                    assert!(
                        !(github_name != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        GITHUB_NAME_ENV_NAME,
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        github_name
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        GITHUB_NAME_ENV_NAME, e
                    );
                }
            }
            match std::env::var(GITHUB_TOKEN_ENV_NAME) {
                Ok(github_token) => {
                    assert!(
                        !(github_token != _USER_CREDENTIALS_DUMMY_HANDLE),
                        "{} != {}, found {}",
                        GITHUB_TOKEN_ENV_NAME,
                        _USER_CREDENTIALS_DUMMY_HANDLE,
                        github_token
                    );
                }
                Err(e) => {
                    panic!(
                        "{} not found in env vars, error: {:#?}",
                        GITHUB_TOKEN_ENV_NAME, e
                    );
                }
            }
        }
        Err(e) => {
            panic!("dotenv() failed error: {:?}", e);
        }
    }
}
