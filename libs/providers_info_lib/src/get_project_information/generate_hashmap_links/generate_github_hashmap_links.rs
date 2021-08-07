use config_lib::get_project_information::get_user_credentials::get_lazy_user_credentials_information::USER_CREDENTIALS;
#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_github_hashmap_links(github_names: Vec<String>) -> Vec<String> {
    //https://github.com/kuqmua.private.atom?token=EXAMPLE_FROM_CONFIG
    let start: &str = "https://github.com/";
    let middle: &str = ".private.atom?token=";
    let mut github_links: Vec<String> = Vec::with_capacity(github_names.len());
    for github_name in github_names {
        let github_link = format!(
            "{}{}{}{}",
            start, github_name, middle, USER_CREDENTIALS.github_authorization.github_token
        );
        github_links.push(github_link);
    }
    github_links
}
