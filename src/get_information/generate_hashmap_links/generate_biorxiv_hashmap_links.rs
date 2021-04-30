use std::collections::HashMap;
pub fn get_biorxiv_links(
    biorxiv_names: HashMap<&'static str, &str>,
) -> HashMap<&'static str, String> {
    let first_part_of_link: &str = "http://connect.biorxiv.org/biorxiv_xml.php?subject=";
    let mut biorxiv_links: HashMap<&str, String> = HashMap::with_capacity(biorxiv_names.len());
    for (key, value) in biorxiv_names {
        biorxiv_links.insert(key, format!("{}{}", first_part_of_link, value));
    }
    biorxiv_links
}
