pub fn subreddits_into_links(subreddits: Vec<&str>) -> Vec<String> {
    let start: &str = "https://www.reddit.com/r/";
    let end: &str = "/new.json";
    let mut subreddits_links = Vec::with_capacity(subreddits.len());
    for subreddit in subreddits {
        let subreddit_link = format!("{}{}{}", start, subreddit, end);
        subreddits_links.push(subreddit_link)
    }
    subreddits_links
}
