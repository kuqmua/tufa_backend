pub fn generate_biorxiv_hashmap_links(biorxiv_names: Vec<String>) -> Vec<String> {
    //example http://connect.biorxiv.org/biorxiv_xml.php?subject=animal_behavior_and_cognition
    let first_part_of_link: &str = "http://connect.biorxiv.org/biorxiv_xml.php?subject=";
    let mut biorxiv_links: Vec<String> = Vec::with_capacity(biorxiv_names.len());
    for value in biorxiv_names {
        biorxiv_links.push(format!("{}{}", first_part_of_link, value));
    }
    biorxiv_links
}
