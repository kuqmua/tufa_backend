use config_lib::get_project_information::get_user_credentials::get_user_credentials_information::USER_CREDENTIALS;

pub fn get_github_names() -> Vec<&'static str> {
    let github_names: Vec<&str> =
        [USER_CREDENTIALS.github_authorization.github_name.as_ref()].to_vec();
    github_names
}
