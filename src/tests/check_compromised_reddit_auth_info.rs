use crate::get_project_information::get_config::config_structures::ConfigStruct;
use crate::tests::tests_constants::VECTOR_OF_MODES;
#[test]
fn check_compromised_reddit_auth_info() {
    for mode in VECTOR_OF_MODES {
        let config_for_test: ConfigStruct =
            ConfigStruct::test_values(*mode).expect("config cannot load config");
        let reddit_user_agent = &config_for_test.reddit_authorization.reddit_user_agent;
        let reddit_client_id = &config_for_test.reddit_authorization.reddit_client_id;
        let reddit_client_secret = &config_for_test.reddit_authorization.reddit_client_secret;
        let reddit_username = &config_for_test.reddit_authorization.reddit_username;
        let reddit_password = &config_for_test.reddit_authorization.reddit_password;
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
