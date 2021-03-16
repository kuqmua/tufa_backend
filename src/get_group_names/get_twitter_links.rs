use std::collections::HashMap;
pub fn get_twitter_links() -> HashMap<&'static str, &'static str> {
    let twitter_sections_links: HashMap<&str, &str> = [
        ("Phys.org", "https://nitter.42l.fr/physorg_com/rss"),
        ("Joe Barnard", "https://nitter.42l.fr/joebarnard/rss"),
    ]
    .iter()
    .cloned()
    .collect();
    twitter_sections_links
}
