use crate::config_mods::lazy_static_config::CONFIG;
use crate::constants::project_constants::GITHUB_LINK_FIRST_PART;
use crate::constants::project_constants::GITHUB_LINK_SECOND_PART;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn generate_github_links(github_names: Vec<String>) -> Vec<String> {
    //https://github.com/kuqmua.private.atom?token=EXAMPLE_FROM_CONFIG
    github_names
        .iter()
        .map(|name| {
            format!(
                "{GITHUB_LINK_FIRST_PART}{name}{GITHUB_LINK_SECOND_PART}{}",
                CONFIG.github_token
            )
        })
        .collect()
}
