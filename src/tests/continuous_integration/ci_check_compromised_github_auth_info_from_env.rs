use crate::get_project_information::env_var_names_constants::GITHUB_NAME_ENV_NAME;
use crate::get_project_information::env_var_names_constants::GITHUB_TOKEN_ENV_NAME;
use crate::get_project_information::get_config::get_lazy_config_information::CONFIG;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_github_auth_info() {
    match std::env::var(GITHUB_NAME_ENV_NAME) {
        Ok(github_name) => {
            if github_name != CONFIG.params.user_credentials_dummy_handle {
                panic!(
                    "{} != {}, found {}", GITHUB_NAME_ENV_NAME,
                    CONFIG.params.user_credentials_dummy_handle,
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
            if github_token != CONFIG.params.user_credentials_dummy_handle {
                panic!(
                    "{} != {}, found {}", GITHUB_TOKEN_ENV_NAME,
                    CONFIG.params.user_credentials_dummy_handle,
                    github_token
                );
            }
        }
        Err(e) => {
            panic!("{} not found in env vars, error: {:#?}", GITHUB_TOKEN_ENV_NAME, e);
        }   
    }
}

