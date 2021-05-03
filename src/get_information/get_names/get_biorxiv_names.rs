use std::collections::HashMap;
pub fn get_biorxiv_names() -> HashMap<&'static str, &'static str> {
    let biorxiv_names: HashMap<&str, &str> = [
        (
            "Animal behavior and cognition",
            "animal_behavior_and_cognition",
        ),
        // ("Biochemistry", "biochemistry"),
        // ("Bioengineering", "bioengineering"),
        // ("Bioinformatics", "bioinformatics"),
        // ("Biophysics", "biophysics"),
        // ("Cancer biology", "cancer_biology"),
        // ("Cell biology", "cell_biology"),
        // ("Clinical trials", "clinical_trials"),
        // ("Developmental biology", "developmental_biology"),
        // ("Ecology", "ecology"),
        // ("Epidemology", "epidemiology"),
        // ("Evolutionary biology", "evolutionary_biology"),
        // ("Genetics", "genetics"),
        // ("Genomics", "genomics"),
        // ("Immunology", "immunology"),
        // ("Microbiology", "microbiology"),
        // ("Molecular biology", "molecular_biology"),
        // ("Neuroscience", "neuroscience"),
        // ("Paleontology", "paleontology"),
        // ("Pathology", "pathology"),
        // ("Pharmacology and toxicology", "pharmacology_and_toxicology"),
        // ("Physiology", "physiology"),
        // ("Plant Biology", "plant_biology"),
        // (
        //     "Scientific communication and education",
        //     "scientific_communication_and_education",
        // ),
        // ("Synthetic Biology", "synthetic_biology"),
        // ("Systems Biology", "systems_biology"),
        // ("Zoology", "zoology"),
    ]
    .iter()
    .cloned()
    .collect();
    biorxiv_names
}
