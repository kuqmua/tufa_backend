use config_lib::get_project_information::get_config::config_structures::ConfigStruct;
use config_lib::get_project_information::get_user_credentials::user_credentials_structures::UserCredentialsStruct;

use config_lib::get_project_information::project_constants::LOAD_CONFIG_FILE_ERROR_MESSAGE;
use config_lib::get_project_information::project_constants::LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE;
use config_lib::get_project_information::project_constants::PATH_TO_CONFIG_FOR_TEST;

use crate::tests::tests_constants::VECTOR_OF_MODES;
#[test]
fn ci_check_compromised_github_auth_info() {
    let user_credentials_for_test: UserCredentialsStruct =
        UserCredentialsStruct::new(PATH_TO_CONFIG_FOR_TEST)
            .expect(LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE);
    for mode in VECTOR_OF_MODES {
        let config_for_test: ConfigStruct = ConfigStruct::new(Some(*mode), PATH_TO_CONFIG_FOR_TEST)
            .expect(LOAD_CONFIG_FILE_ERROR_MESSAGE);
        let github_name = &user_credentials_for_test.github_authorization.github_name;
        let github_name = &user_credentials_for_test.github_authorization.github_token;
        if github_name != &config_for_test.params.user_credentials_dummy_handle {
            panic!(
                "github_name != {} for mode {}",
                &config_for_test.params.user_credentials_dummy_handle, mode
            );
        }
        if github_name != &config_for_test.params.user_credentials_dummy_handle {
            panic!(
                "github_name != {} for mode {}",
                &config_for_test.params.user_credentials_dummy_handle, mode
            );
        }
    }
}
