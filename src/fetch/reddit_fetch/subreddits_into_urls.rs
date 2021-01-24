pub fn subreddits_into_urls(subreddits: Vec<&str>) -> Vec<String> {
    let start: &str = "https://www.reddit.com/r/";
    let end: &str = "/new.json";
    let mut subreddits_urls = Vec::with_capacity(subreddits.len());
    for subreddit in subreddits {
        let subreddit_url = format!("{}{}{}", start, subreddit, end);
        subreddits_urls.push(subreddit_url)
    }
    subreddits_urls
}
