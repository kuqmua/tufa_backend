use crate::config_mods::get_config::config_struct::ConfigStruct;

use crate::constants::project_constants::LOAD_CONFIG_FILE_ERROR_MESSAGE;

use crate::constants::tests_constants::_USER_CREDENTIALS_DUMMY_HANDLE;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_github_auth_info_with_config_init() {
    let config_handle: ConfigStruct = ConfigStruct::new().expect(LOAD_CONFIG_FILE_ERROR_MESSAGE);
    let github_name = &config_handle.github_authorization.github_name;
    let github_token = &config_handle.github_authorization.github_token;
    if github_name != _USER_CREDENTIALS_DUMMY_HANDLE {
        panic!("github_name != {}", _USER_CREDENTIALS_DUMMY_HANDLE);
    }
    if github_token != _USER_CREDENTIALS_DUMMY_HANDLE {
        panic!("github_token != {}", _USER_CREDENTIALS_DUMMY_HANDLE);
    }
}
