use crate::constants::project_constants::BIORXIV_LINK_FIRST_PART;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_biorxiv_links(biorxiv_names: Vec<String>) -> Vec<String> {
    //example http://connect.biorxiv.org/biorxiv_xml.php?subject=animal_behavior_and_cognition
    biorxiv_names
        .iter()
        .map(|name| format!("{}{}", BIORXIV_LINK_FIRST_PART, name))
        .collect()
}
