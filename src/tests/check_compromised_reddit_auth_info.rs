use crate::get_project_information::get_config::config_structures::ConfigStruct;
#[test]
fn check_compromised_reddit_auth_info() {
    let vec_of_modes: Vec<&str> = vec!["Default", "Development", "Production", "Testing"];
    for mode in vec_of_modes {
        let config_for_test: ConfigStruct =
            ConfigStruct::test_values(mode).expect("config can be loaded");
        let reddit_user_agent = &config_for_test.reddit_authorization.reddit_user_agent;
        let reddit_client_id = &config_for_test.reddit_authorization.reddit_client_id;
        let reddit_client_secret = &config_for_test.reddit_authorization.reddit_client_secret;
        let reddit_username = &config_for_test.reddit_authorization.reddit_username;
        let reddit_password = &config_for_test.reddit_authorization.reddit_password;
        let example: &str = "example";
        if reddit_user_agent != example {
            panic!("reddit_user_agent != {} for mode {}", "example", mode)
        }
        if reddit_client_id != example {
            panic!("reddit_client_id != {} for mode {}", "example", mode)
        }
        if reddit_client_secret != example {
            panic!("reddit_client_secret != {} for mode {}", "example", mode)
        }
        if reddit_username != example {
            panic!("reddit_username != {} for mode {}", "example", mode)
        }
        if reddit_password != example {
            panic!("reddit_password != {} for mode {}", "example", mode)
        }
    }
}
