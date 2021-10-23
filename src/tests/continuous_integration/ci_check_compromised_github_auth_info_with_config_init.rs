use crate::get_project_information::get_config::config_struct::ConfigStruct;

use crate::get_project_information::project_constants::LOAD_CONFIG_FILE_ERROR_MESSAGE;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_github_auth_info_with_config_init() {
            let config_handle: ConfigStruct = ConfigStruct::new()
            .expect(LOAD_CONFIG_FILE_ERROR_MESSAGE);
        let github_name = &config_handle.github_authorization.github_name;
        let github_token = &config_handle.github_authorization.github_token;
        if github_name != &config_handle.params.user_credentials_dummy_handle {
            panic!(
                "github_name != {}",
                &config_handle.params.user_credentials_dummy_handle
            );
        }
        if github_token != &config_handle.params.user_credentials_dummy_handle {
            panic!(
                "github_token != {}",
                &config_handle.params.user_credentials_dummy_handle
            );
        }
}
