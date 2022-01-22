use crate::constants::project_constants::ARXIV_LINK_FIRST_PART;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_arxiv_links(arxiv_names: Vec<String>) -> Vec<String> {
    //example http://export.arxiv.org/rss/astro-ph.CO
    arxiv_names
        .iter()
        .map(|name| format!("{}{}", ARXIV_LINK_FIRST_PART, name))
        .collect()
}
