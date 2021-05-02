use std::collections::HashMap;
pub fn generate_arxiv_hashmap_links(
    arxiv_names: HashMap<&'static str, &str>,
) -> HashMap<&'static str, String> {
    let first_part_of_link: &str = "http://export.arxiv.org/rss/";
    let mut arxiv_links: HashMap<&str, String> = HashMap::with_capacity(arxiv_names.len());
    for (key, value) in arxiv_names {
        arxiv_links.insert(key, format!("{}{}", first_part_of_link, value));
    }
    arxiv_links
}
