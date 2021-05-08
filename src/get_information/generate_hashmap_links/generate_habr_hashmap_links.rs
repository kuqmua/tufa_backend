use std::collections::HashMap;
pub fn generate_habr_hashmap_links(habr_names: Vec<&'static str>) -> HashMap<&'static str, String> {
    //example https://habr.com/ru/rss/all/all/?fl=ru?with_hubs=true:?with_tags=true:
    let habr_key = "Habr";
    let habr_link = "https://habr.com/ru/rss/";
    let mut habr_links: HashMap<&str, String> = HashMap::with_capacity(habr_names.len());
    for habr_name in habr_names {
        let habr_link_full = format!("{}{}", habr_link, habr_name);
        habr_links.insert(habr_key, habr_link_full);
    }
    habr_links
}
