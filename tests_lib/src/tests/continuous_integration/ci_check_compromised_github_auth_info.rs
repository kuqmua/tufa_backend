use crate::tests::tests_constants::USER_CREDENTIALS_DUMMY_HANDLE;
use crate::tests::tests_constants::GITHUB_NAME_ENV_NAME;
use crate::tests::tests_constants::GITHUB_TOKEN_ENV_NAME;


#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_github_auth_info() {
    match std::env::var(GITHUB_NAME_ENV_NAME) {
        Ok(github_name) => {
            if github_name != USER_CREDENTIALS_DUMMY_HANDLE {
                panic!(
                    "{} != {}, found {}", GITHUB_NAME_ENV_NAME,
                    USER_CREDENTIALS_DUMMY_HANDLE,
                    github_name
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", GITHUB_NAME_ENV_NAME, e);
        }   
    }
    match std::env::var(GITHUB_TOKEN_ENV_NAME) {
        Ok(github_token) => {
            if github_token != USER_CREDENTIALS_DUMMY_HANDLE {
                panic!(
                    "{} != {}, found {}", GITHUB_TOKEN_ENV_NAME,
                    USER_CREDENTIALS_DUMMY_HANDLE,
                    github_token
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", GITHUB_TOKEN_ENV_NAME, e);
        }   
    }
}

