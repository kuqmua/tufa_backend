#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_reddit_hashmap_links(subreddits_names: Vec<String>) -> Vec<String> {
    //example https://www.reddit.com/r/3Dprinting/new.json
    let start: &str = "https://www.reddit.com/r/";
    let end: &str = "/new.json";
    let mut reddit_links: Vec<String> = Vec::with_capacity(subreddits_names.len());
    for subreddit_name in subreddits_names {
        let subreddit_link = format!("{}{}{}", start, subreddit_name, end);
        reddit_links.push(subreddit_link);
    }
    reddit_links
}
