use crate::get_project_information::get_config::config_structures::ConfigStruct;
const USER_CREDENTIALS_DUMMY_HANDLE: &str = "example";
#[test]
fn check_compromised_reddit_auth_info() {
    let vec_of_modes: Vec<&str> = vec!["Default", "Development", "Production", "Testing"];
    for mode in vec_of_modes {
        let config_for_test: ConfigStruct =
            ConfigStruct::test_values(mode).expect("config cannot load config");
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
