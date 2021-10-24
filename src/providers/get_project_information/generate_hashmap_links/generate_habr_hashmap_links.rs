#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_habr_hashmap_links(habr_names: Vec<String>) -> Vec<String> {
    //example https://habr.com/ru/rss/all/all/?fl=ru?with_hubs=true:?with_tags=true:
    let habr_link = "https://habr.com/ru/rss/";
    let mut habr_links: Vec<String> = Vec::with_capacity(habr_names.len());
    for habr_name in habr_names {
        let habr_link_full = format!("{}{}", habr_link, habr_name);
        habr_links.push(habr_link_full);
    }
    habr_links
}
