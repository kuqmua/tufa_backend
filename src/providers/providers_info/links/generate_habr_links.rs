use crate::constants::project_constants::HABR_LINK_FIRST_PART;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_habr_links(habr_names: Vec<String>) -> Vec<String> {
    //example https://habr.com/ru/rss/all/all/?fl=ru?with_hubs=true:?with_tags=true:
    let mut habr_links: Vec<String> = Vec::with_capacity(habr_names.len());
    for habr_name in habr_names {
        habr_links.push(format!("{}{}", HABR_LINK_FIRST_PART, habr_name));
    }
    habr_links
}
