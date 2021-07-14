pub fn generate_medrxiv_hashmap_links(medrxiv_names: Vec<String>) -> Vec<String> {
    //example http://connect.medrxiv.org/medrxiv_xml.php?subject=Addiction_Medicine
    let first_part_of_link: &str = "http://connect.medrxiv.org/medrxiv_xml.php?subject=";
    let mut medrxiv_links: Vec<String> = Vec::with_capacity(medrxiv_names.len());
    for value in medrxiv_names {
        medrxiv_links.push(format!("{}{}", first_part_of_link, value));
    }
    medrxiv_links
}
