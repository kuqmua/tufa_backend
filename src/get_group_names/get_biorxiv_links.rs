use std::collections::HashMap;
pub fn get_biorxiv_links() -> HashMap<&'static str, &'static str> {
    let arxiv_sections_links: HashMap<&str,&str> =
    [("Animal behavior and cognition","http://connect.biorxiv.org/biorxiv_xml.php?subject=animal_behavior_and_cognition"),
     ("Biochemistry","http://connect.biorxiv.org/biorxiv_xml.php?subject=biochemistry"),
     ("Bioengineering","http://connect.biorxiv.org/biorxiv_xml.php?subject=bioengineering"),
     ("Bioinformatics","http://connect.biorxiv.org/biorxiv_xml.php?subject=bioinformatics"),
     ("Biophysics","http://connect.biorxiv.org/biorxiv_xml.php?subject=biophysics"),
     ("Cancer biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=cancer_biology"),
     ("Cell biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=cell_biology"),
     ("Clinical trials","http://connect.biorxiv.org/biorxiv_xml.php?subject=clinical_trials"),
     ("Developmental biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=developmental_biology"),
     ("Ecology","http://connect.biorxiv.org/biorxiv_xml.php?subject=ecology"),
     ("Epidemology","http://connect.biorxiv.org/biorxiv_xml.php?subject=epidemiology"),
     ("Evolutionary biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=evolutionary_biology"),
     ("Genetics","http://connect.biorxiv.org/biorxiv_xml.php?subject=genetics"),
     ("Genomics","http://connect.biorxiv.org/biorxiv_xml.php?subject=genomics"),
     ("Immunology","http://connect.biorxiv.org/biorxiv_xml.php?subject=immunology"),
     ("Microbiology","http://connect.biorxiv.org/biorxiv_xml.php?subject=microbiology"),
     ("Molecular biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=molecular_biology"),
     ("Neuroscience","http://connect.biorxiv.org/biorxiv_xml.php?subject=neuroscience"),
    ("Paleontology","http://connect.biorxiv.org/biorxiv_xml.php?subject=paleontology"),
    ("Pathology","http://connect.biorxiv.org/biorxiv_xml.php?subject=pathology"),
    ("Pharmacology and toxicology","http://connect.biorxiv.org/biorxiv_xml.php?subject=pharmacology_and_toxicology"),
    ("Physiology","http://connect.biorxiv.org/biorxiv_xml.php?subject=physiology"),
    ("Plant Biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=plant_biology"),
    ("Scientific communication and education","http://connect.biorxiv.org/biorxiv_xml.php?subject=scientific_communication_and_education"),
    ("Synthetic Biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=synthetic_biology"),
    ("Systems Biology","http://connect.biorxiv.org/biorxiv_xml.php?subject=systems_biology"),
    ("Zoology","http://connect.biorxiv.org/biorxiv_xml.php?subject=zoology"),
     ]
     .iter().cloned().collect();
    arxiv_sections_links
}
