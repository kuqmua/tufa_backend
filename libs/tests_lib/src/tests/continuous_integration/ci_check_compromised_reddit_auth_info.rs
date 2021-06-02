use get_config_lib::get_project_information::get_config::config_structures::ConfigStruct;
use get_config_lib::get_project_information::get_user_credentials::user_credentials_structures::UserCredentialsStruct;

use get_config_lib::get_project_information::project_constants::LOAD_CONFIG_FILE_ERROR_MESSAGE;
use get_config_lib::get_project_information::project_constants::LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE;
use get_config_lib::get_project_information::project_constants::PATH_TO_CONFIG_FOR_TEST;

use crate::tests::tests_constants::VECTOR_OF_MODES;
#[test]
fn ci_check_compromised_reddit_auth_info() {
    let user_credentials_for_test: UserCredentialsStruct =
        UserCredentialsStruct::new(PATH_TO_CONFIG_FOR_TEST)
            .expect(LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE);
    for mode in VECTOR_OF_MODES {
        let config_for_test: ConfigStruct = ConfigStruct::new(Some(*mode), PATH_TO_CONFIG_FOR_TEST)
            .expect(LOAD_CONFIG_FILE_ERROR_MESSAGE);
        let reddit_user_agent = &user_credentials_for_test
            .reddit_authorization
            .reddit_user_agent;
        let reddit_client_id = &user_credentials_for_test
            .reddit_authorization
            .reddit_client_id;
        let reddit_client_secret = &user_credentials_for_test
            .reddit_authorization
            .reddit_client_secret;
        let reddit_username = &user_credentials_for_test
            .reddit_authorization
            .reddit_username;
        let reddit_password = &user_credentials_for_test
            .reddit_authorization
            .reddit_password;
        if reddit_user_agent != &config_for_test.params.user_credentials_dummy_handle {
            panic!(
                "reddit_user_agent != {} for mode {}",
                &config_for_test.params.user_credentials_dummy_handle, mode
            );
        }
        if reddit_client_id != &config_for_test.params.user_credentials_dummy_handle {
            panic!(
                "reddit_client_id != {} for mode {}",
                &config_for_test.params.user_credentials_dummy_handle, mode
            );
        }
        if reddit_client_secret != &config_for_test.params.user_credentials_dummy_handle {
            panic!(
                "reddit_client_secret != {} for mode {}",
                &config_for_test.params.user_credentials_dummy_handle, mode
            );
        }
        if reddit_username != &config_for_test.params.user_credentials_dummy_handle {
            panic!(
                "reddit_username != {} for mode {}",
                &config_for_test.params.user_credentials_dummy_handle, mode
            );
        }
        if reddit_password != &config_for_test.params.user_credentials_dummy_handle {
            panic!(
                "reddit_password != {} for mode {}",
                &config_for_test.params.user_credentials_dummy_handle, mode
            );
        }
    }
}
