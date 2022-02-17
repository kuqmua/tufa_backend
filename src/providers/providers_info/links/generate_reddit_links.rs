use crate::constants::project_constants::REDDIT_LINK_FIRST_PART;
use crate::constants::project_constants::REDDIT_LINK_SECOND_PART;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_reddit_links(subreddits_names: Vec<String>) -> Vec<String> {
    //example https://www.reddit.com/r/3Dprinting/new.json
    subreddits_names
        .iter()
        .map(|name| {
            format!(
                "{REDDIT_LINK_FIRST_PART}{name}{REDDIT_LINK_SECOND_PART}")
        })
        .collect()
}
