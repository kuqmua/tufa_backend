use crate::config_mods::lazy_static_config::CONFIG;

use crate::constants::project_constants::GITHUB_LINK_FIRST_PART;
use crate::constants::project_constants::GITHUB_LINK_SECOND_PART;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_github_links(github_names: Vec<String>) -> Vec<String> {
    //https://github.com/kuqmua.private.atom?token=EXAMPLE_FROM_CONFIG
    let mut github_links: Vec<String> = Vec::with_capacity(github_names.len());
    for github_name in github_names {
        github_links.push(format!(
            "{}{}{}{}",
            GITHUB_LINK_FIRST_PART,
            github_name,
            GITHUB_LINK_SECOND_PART,
            CONFIG.github_authorization.github_token
        ));
    }
    github_links
}
