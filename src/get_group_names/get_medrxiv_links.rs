use std::collections::HashMap;
pub fn get_medrxiv_links() -> HashMap<&'static str, String> {
    let medrxiv_sections: Vec<(&str, &str)> = vec![
        ("Addiction Medicine", "Addiction_Medicine"),
        ("Allergy and Immunology", "Allergy_and_Immunology"),
        ("Anesthesia", "Anesthesia"),
        // ("Cardiovascular Medicine", "Cardiovascular_Medicine"),
        // ("Dentistry and Oral Medicine", "Dentistry_and_Oral_Medicine"),
        // ("Dermatology", "Dermatology"),
        // ("Emergency Medicine", "Emergency_Medicine"),
        // ("Endocrinology", "endocrinology"),
        // ("Epidemiology", "Epidemiology"),
        // ("Forensic Medicine", "Forensic_Medicine"),
        // ("Gastroenterology", "Gastroenterology"),
        // (
        //     "Genetic and Genomic Medicine",
        //     "Genetic_and_Genomic_Medicine",
        // ),
        // ("Geriatric Medicine", "Geriatric_Medicine"),
        // ("Health Economics", "Health_Economics"),
        // ("Health Informatics", "Health_Informatics"),
        // ("Health Policy", "Health_Policy"),
        // (
        //     "Health Systems and Quality Improvement",
        //     "Health_Systems_and_Quality_Improvement",
        // ),
        // ("Hematology", "Hematology"),
        // ("HIV/AIDS", "hivaids"), //тут чет иногда ничего нет
        // ("Infectious Diseases", "infectious_diseases"),
        // (
        //     "Intensive Care and Critical Care Medicine",
        //     "Intensive_Care_and_Critical_Care_Medicine",
        // ),
        // ("Medical Education", "Medical_Education"),
        // ("Medical Ethics", "Medical_Ethics"),
        // ("Nephrology", "Nephrology"),
        // ("Neurology", "Neurology"),
        // ("Nursing", "Nursing"),
        // ("Nutrition", "Nutrition"),
        // ("Obstetrics and Gynecology", "Obstetrics_and_Gynecology"),
        // (
        //     "Occupational and Environmental Health",
        //     "Occupational_and_Environmental_Health",
        // ),
        // ("Oncology", "Oncology"),
        // ("Ophthalmology", "Ophthalmology"),
        // ("Orthopedics", "Orthopedics"),
        // ("Otolaryngology", "Otolaryngology"),
        // ("Pain Medicine", "Pain_Medicine"),
        // ("Palliative Medicine", "Palliative_Medicine"),
        // ("Pathology", "Pathology"),
        // ("Pediatrics", "Pediatrics"),
        // (
        //     "Pharmacology and Therapeutics",
        //     "Pharmacology_and_Therapeutics",
        // ),
        // ("Primary Care Research", "Primary_Care_Research"),
        // (
        //     "Psychiatry and Clinical Psychology",
        //     "Psychiatry_and_Clinical_Psychology",
        // ),
        // ("Public and Global Health", "Public_and_Global_Health"),
        // ("Radiology and Imaging", "Radiology_and_Imaging"),
        // (
        //     "Rehabilitation Medicine and Physical Therapy",
        //     "Rehabilitation_Medicine_and_Physical_Therapy",
        // ),
        // ("Respiratory Medicine", "Respiratory_Medicine"),
        // ("Rheumatology", "Rheumatology"),
        // (
        //     "Sexual and Reproductive Health",
        //     "Sexual_and_Reproductive_Health",
        // ),
        // ("Sports Medicine", "Sports_Medicine"),
        // ("Surgery", "Surgery"),
        // ("Toxicology", "Toxicology"),
        // ("Transplantation", "Transplantation"),
        // ("Urology", "Urology"),
    ]
    .to_vec();
    let first_part_of_link: &str = "http://connect.medrxiv.org/medrxiv_xml.php?subject=";
    let mut medrxiv_sections_links: HashMap<&'static str, String> =
        HashMap::with_capacity(medrxiv_sections.len());
    for (key, value) in medrxiv_sections {
        medrxiv_sections_links.insert(key, format!("{}{}", first_part_of_link, value));
    }
    medrxiv_sections_links
}
