use crate::constants::project_constants::REDDIT_LINK_FIRST_PART;
use crate::constants::project_constants::REDDIT_LINK_SECOND_PART;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_reddit_links(subreddits_names: Vec<String>) -> Vec<String> {
    //example https://www.reddit.com/r/3Dprinting/new.json
    let mut reddit_links: Vec<String> = Vec::with_capacity(subreddits_names.len());
    for subreddit_name in subreddits_names {
        let subreddit_link = format!(
            "{}{}{}",
            REDDIT_LINK_FIRST_PART, subreddit_name, REDDIT_LINK_SECOND_PART
        );
        reddit_links.push(subreddit_link);
    }
    reddit_links
}
