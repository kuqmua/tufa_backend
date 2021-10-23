use config_lib::get_project_information::get_config::config_struct::ConfigStruct;

use config_lib::get_project_information::project_constants::LOAD_CONFIG_FILE_ERROR_MESSAGE;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_reddit_auth_info() {
    let config_handle: ConfigStruct =
        ConfigStruct::new()
            .expect(LOAD_CONFIG_FILE_ERROR_MESSAGE);
        let reddit_user_agent = &config_handle
            .reddit_authorization
            .reddit_user_agent;
        let reddit_client_id = &config_handle
            .reddit_authorization
            .reddit_client_id;
        let reddit_client_secret = &config_handle
            .reddit_authorization
            .reddit_client_secret;
        let reddit_username = &config_handle
            .reddit_authorization
            .reddit_username;
        let reddit_password = &config_handle
            .reddit_authorization
            .reddit_password;
        if reddit_user_agent != &config_handle.params.user_credentials_dummy_handle {
            panic!(
                "reddit_user_agent != {}",
                &config_handle.params.user_credentials_dummy_handle
            );
        }
        if reddit_client_id != &config_handle.params.user_credentials_dummy_handle {
            panic!(
                "reddit_client_id != {}",
                &config_handle.params.user_credentials_dummy_handle
            );
        }
        if reddit_client_secret != &config_handle.params.user_credentials_dummy_handle {
            panic!(
                "reddit_client_secret != {}",
                &config_handle.params.user_credentials_dummy_handle
            );
        }
        if reddit_username != &config_handle.params.user_credentials_dummy_handle {
            panic!(
                "reddit_username != {}",
                &config_handle.params.user_credentials_dummy_handle
            );
        }
        if reddit_password != &config_handle.params.user_credentials_dummy_handle {
            panic!(
                "reddit_password != {}",
                &config_handle.params.user_credentials_dummy_handle
            );
        }
}
