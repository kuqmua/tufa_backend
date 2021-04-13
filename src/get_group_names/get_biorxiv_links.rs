use std::collections::HashMap;
pub fn get_biorxiv_links() -> HashMap<&'static str, String> {
    let biorxiv_sections: HashMap<&str, &str> = [
        (
            "Animal behavior and cognition",
            "animal_behavior_and_cognition",
        ),
        ("Biochemistry", "biochemistry"),
        ("Bioengineering", "bioengineering"),
        ("Bioinformatics", "bioinformatics"),
        ("Biophysics", "biophysics"),
        ("Cancer biology", "cancer_biology"),
        ("Cell biology", "cell_biology"),
        ("Clinical trials", "clinical_trials"),
        ("Developmental biology", "developmental_biology"),
        ("Ecology", "ecology"),
        ("Epidemology", "epidemiology"),
        ("Evolutionary biology", "evolutionary_biology"),
        ("Genetics", "genetics"),
        ("Genomics", "genomics"),
        ("Immunology", "immunology"),
        ("Microbiology", "microbiology"),
        ("Molecular biology", "molecular_biology"),
        ("Neuroscience", "neuroscience"),
        ("Paleontology", "paleontology"),
        ("Pathology", "pathology"),
        ("Pharmacology and toxicology", "pharmacology_and_toxicology"),
        ("Physiology", "physiology"),
        ("Plant Biology", "plant_biology"),
        (
            "Scientific communication and education",
            "scientific_communication_and_education",
        ),
        ("Synthetic Biology", "synthetic_biology"),
        ("Systems Biology", "systems_biology"),
        ("Zoology", "zoology"),
    ]
    .iter()
    .cloned()
    .collect();
    let first_part_of_link: &str = "http://connect.biorxiv.org/biorxiv_xml.php?subject=";
    let mut biorxiv_sections_links: HashMap<&str, String> =
        HashMap::with_capacity(biorxiv_sections.len());
    for (key, value) in biorxiv_sections {
        biorxiv_sections_links.insert(key, format!("{}{}", first_part_of_link, value));
    }
    biorxiv_sections_links
}
