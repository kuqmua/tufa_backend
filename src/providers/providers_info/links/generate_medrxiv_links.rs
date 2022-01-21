use crate::constants::project_constants::MEDRXIV_LINK_FIRST_PART;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn generate_medrxiv_links(medrxiv_names: Vec<String>) -> Vec<String> {
    //example http://connect.medrxiv.org/medrxiv_xml.php?subject=Addiction_Medicine
    medrxiv_names.iter().map(|name| format!("{}{}", MEDRXIV_LINK_FIRST_PART, name)).collect()
}
