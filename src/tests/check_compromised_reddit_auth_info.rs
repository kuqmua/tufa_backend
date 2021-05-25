use crate::get_project_information::get_config::config_structures::ConfigStruct;
use crate::project_constants::USER_CREDENTIALS_DUMMY_HANDLE;
use crate::project_constants::VECTOR_OF_MODES;
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
        if reddit_user_agent != USER_CREDENTIALS_DUMMY_HANDLE {
            panic!(
                "reddit_user_agent != {} for mode {}",
                USER_CREDENTIALS_DUMMY_HANDLE, mode
            );
        }
        if reddit_client_id != USER_CREDENTIALS_DUMMY_HANDLE {
            panic!(
                "reddit_client_id != {} for mode {}",
                USER_CREDENTIALS_DUMMY_HANDLE, mode
            );
        }
        if reddit_client_secret != USER_CREDENTIALS_DUMMY_HANDLE {
            panic!(
                "reddit_client_secret != {} for mode {}",
                USER_CREDENTIALS_DUMMY_HANDLE, mode
            );
        }
        if reddit_username != USER_CREDENTIALS_DUMMY_HANDLE {
            panic!(
                "reddit_username != {} for mode {}",
                USER_CREDENTIALS_DUMMY_HANDLE, mode
            );
        }
        if reddit_password != USER_CREDENTIALS_DUMMY_HANDLE {
            panic!(
                "reddit_password != {} for mode {}",
                USER_CREDENTIALS_DUMMY_HANDLE, mode
            );
        }
    }
}
