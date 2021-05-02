use std::collections::HashMap;

pub fn generate_reddit_hashmap_links(
    subreddits_names: Vec<&'static str>,
) -> HashMap<&'static str, String> {
    let start: &str = "https://www.reddit.com/r/";
    let end: &str = "/new.json";
    let mut reddit_links: HashMap<&str, String> = HashMap::with_capacity(subreddits_names.len());
    for subreddit_name in subreddits_names {
        let subreddit_link = format!("{}{}{}", start, subreddit_name, end);
        reddit_links.insert(subreddit_name, subreddit_link);
    }
    reddit_links
}
