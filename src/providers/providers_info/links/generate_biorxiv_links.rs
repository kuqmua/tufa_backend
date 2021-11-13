use crate::constants::project_constants::BIORXIV_LINK_FIRST_PART;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_biorxiv_links(biorxiv_names: Vec<String>) -> Vec<String> {
    //example http://connect.biorxiv.org/biorxiv_xml.php?subject=animal_behavior_and_cognition
    let mut biorxiv_links: Vec<String> = Vec::with_capacity(biorxiv_names.len());
    for value in biorxiv_names {
        biorxiv_links.push(format!("{}{}", BIORXIV_LINK_FIRST_PART, value));
    }
    biorxiv_links
}
