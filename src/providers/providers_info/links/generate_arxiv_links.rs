use crate::constants::project_constants::ARXIV_LINK_FIRST_PART;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_arxiv_links(arxiv_names: Vec<String>) -> Vec<String> {
    //example http://export.arxiv.org/rss/astro-ph.CO
    let mut arxiv_links: Vec<String> = Vec::with_capacity(arxiv_names.len());
    for value in arxiv_names {
        arxiv_links.push(format!("{}{}", ARXIV_LINK_FIRST_PART, value));
    }
    arxiv_links
}
