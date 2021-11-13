use crate::constants::project_constants::MEDRXIV_LINK_FIRST_PART;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_medrxiv_links(medrxiv_names: Vec<String>) -> Vec<String> {
    //example http://connect.medrxiv.org/medrxiv_xml.php?subject=Addiction_Medicine
    let mut medrxiv_links: Vec<String> = Vec::with_capacity(medrxiv_names.len());
    for value in medrxiv_names {
        medrxiv_links.push(format!("{}{}", MEDRXIV_LINK_FIRST_PART, value));
    }
    medrxiv_links
}
