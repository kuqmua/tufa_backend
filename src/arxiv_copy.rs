extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;
// extern crate xml;
use futures::future;
use reqwest::Client;
use serde_xml_rs::from_str;
use std::collections::HashMap;
use std::str;
use std::time::Instant;
use tokio;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlArxivParserStruct {
    #[serde(rename = "item", default)]
    pub items: Vec<XmlArxivParserPost>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct XmlArxivParserPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creator: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivPostStruct {
    pub items: Vec<ArxivPost>,
}
//count: usize
impl ArxivPostStruct {
    pub fn new() -> Self {
        ArxivPostStruct {
            items: Vec::<ArxivPost>::new(),
            // items: vec![ArxivPost::new(); count],
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creators: Vec<Creator>,
}

impl ArxivPost {
    pub fn new() -> Self {
        ArxivPost {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            creators: Vec::<Creator>::new(),
            // creators: vec![Creator::new(); 70],
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Creator {
    pub name: String,
    pub link: String,
}

impl Creator {
    pub fn new() -> Self {
        Creator {
            name: "".to_string(),
            link: "".to_string(),
        }
    }
}

#[tokio::main]
pub async fn fetch_and_parse_xml_biorxiv(
    vec_of_links: Vec<&str>,
    vec_of_keys: Vec<&str>,
) -> HashMap<String, ArxivPostStruct> {
    let time = Instant::now();
    let mut biorxiv_structs_vec: HashMap<String, ArxivPostStruct> =
        HashMap::with_capacity(vec_of_links.len());
    let client = Client::new();
    println!("starting fetching arxiv...");
    let bodies = future::join_all(vec_of_links.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    }))
    .await;
    println!(
        "arxiv future::join_all (in seconds) = {} ",
        time.elapsed().as_secs()
    );
    println!("arxiv bodies.len() {}", bodies.len());
    let mut key_count = 0;
    for b in bodies {
        //подсчитать на новом компе сколько эта операция будет занимать времени. если долго то распаралелить
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let converted_str = str::from_utf8(&slice).unwrap();
                let dots_unfiltered_str = converted_str.to_string();
                // расписать случай если не найдет посты
                match dots_unfiltered_str.find("</item>") {
                    Some(_) => {
                        let biorvix_struct: XmlArxivParserStruct =
                            from_str(&dots_unfiltered_str).unwrap();
                        let mut count = 0;
                        let mut biorxiv_page_struct: ArxivPostStruct = ArxivPostStruct::new();
                        //biorvix_struct.items.len()
                        loop {
                            if count < biorvix_struct.items.len() {
                                let mut arxiv_post: ArxivPost = ArxivPost::new();
                                arxiv_post.title = biorvix_struct.items[count].title.clone();
                                arxiv_post.link = biorvix_struct.items[count].link.clone();
                                arxiv_post.description =
                                    biorvix_struct.items[count].description.clone();
                                let mut string_part_for_loop =
                                    biorvix_struct.items[count].creator.clone();
                                while let Some(link_index_from_start) =
                                    string_part_for_loop.find("<a href=\"")
                                {
                                    if let Some(link_index_from_end) =
                                        string_part_for_loop.find("\">")
                                    {
                                        if let Some(name_index_from_end) =
                                            string_part_for_loop.find("</a>")
                                        {
                                            let mut creator = Creator::new();
                                            creator.link = string_part_for_loop
                                                [link_index_from_start + "<a href=\"".len()
                                                    ..link_index_from_end]
                                                .to_string();
                                            let name_index_from_start =
                                                link_index_from_end + "\">".len();
                                            creator.name = string_part_for_loop
                                                [name_index_from_start..name_index_from_end]
                                                .to_string();
                                            string_part_for_loop = string_part_for_loop
                                                [name_index_from_end + "\">".len()..]
                                                .to_string();
                                            arxiv_post.creators.push(creator);
                                        }
                                    }
                                }
                                biorxiv_page_struct.items.push(arxiv_post);
                                count += 1;
                            } else {
                                break;
                            }
                        }
                        biorxiv_structs_vec
                            .insert(vec_of_keys[key_count].to_string(), biorxiv_page_struct);
                    }
                    _ => {
                        println!("(arxiv) no items for key {}", vec_of_keys[key_count]);
                        let useless_biorxiv_page_struct = ArxivPostStruct::new();
                        biorxiv_structs_vec.insert(
                            vec_of_keys[key_count].to_string(),
                            useless_biorxiv_page_struct,
                        );
                    }
                }
            }
            Err(e) => {
                println!("key_count on EERRRRRRRRRROOOOORRRRR {}", key_count);
                eprintln!("Got an error: {}", e);
            }
        }
        key_count += 1;
    }
    println!(
        "arxiv xml parsing (in seconds) = {} ",
        time.elapsed().as_secs()
    );
    biorxiv_structs_vec.clone()
}

pub fn arxiv_part() -> HashMap<String, ArxivPostStruct> {
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links_in_hash_map();
    println!(
        "{:#?} elements in Arxiv HashMap",
        arxiv_links_in_hash_map.len()
    );
    let vec_of_links: Vec<&str> = arxiv_links_in_hash_map.values().cloned().collect();
    let vec_of_keys: Vec<&str> = arxiv_links_in_hash_map.keys().cloned().collect();
    let vec_of_vec_of_strings = fetch_and_parse_xml_biorxiv(vec_of_links, vec_of_keys);
    vec_of_vec_of_strings
}

pub fn get_arxiv_links_in_hash_map() -> HashMap<&'static str, &'static str> {
    let arxiv_sections_links: HashMap<&str, &str> = [
        (
            "Cosmology and Nongalactic Astrophysics",
            "http://export.arxiv.org/rss/astro-ph.CO",
        ),
        (
            "Earth and Planetary Astrophysics",
            "http://export.arxiv.org/rss/astro-ph.EP",
        ),
        (
            "Astrophysics of Galaxies",
            "http://export.arxiv.org/rss/astro-ph.GA",
        ),
        (
            "High Energy Astrophysical Phenomena",
            "http://export.arxiv.org/rss/astro-ph.HE",
        ),
        (
            "Instrumentation and Methods for Astrophysics",
            "http://export.arxiv.org/rss/astro-ph.IM",
        ),
        (
            "Solar and Stellar Astrophysics",
            "http://export.arxiv.org/rss/astro-ph.SR",
        ),
        //Condensed Matter
        (
            "Disordered Systems and Neural Networks",
            "http://export.arxiv.org/rss/cond-mat.dis-nn",
        ),
        (
            "Mesoscale and Nanoscale Physics",
            "http://export.arxiv.org/rss/cond-mat.mes-hall",
        ),
        (
            "Materials Science",
            "http://export.arxiv.org/rss/cond-mat.mtrl-sci",
        ),
        (
            "Other Condensed Matter",
            "http://export.arxiv.org/rss/cond-mat.other",
        ), //НАДО ЛИ - УПАДЕТ ЖЕ
        (
            "Quantum Gases",
            "http://export.arxiv.org/rss/cond-mat.quant-gas",
        ),
        (
            "Soft Condensed Matter",
            "http://export.arxiv.org/rss/cond-mat.soft",
        ),
        (
            "Statistical Mechanics",
            "http://export.arxiv.org/rss/cond-mat.stat-mech",
        ),
        (
            "Strongly Correlated Electrons",
            "http://export.arxiv.org/rss/cond-mat.str-el",
        ),
        (
            "Superconductivity",
            "http://export.arxiv.org/rss/cond-mat.supr-con",
        ),
        //Computer Science
        (
            "Artificial Intelligence",
            "http://export.arxiv.org/rss/cs.AI",
        ),
        ("Hardware Architecture", "http://export.arxiv.org/rss/cs.AR"),
        (
            "Computational Complexity",
            "http://export.arxiv.org/rss/cs.CC",
        ),
        (
            "Computational Engineering, Finance, and Science",
            "http://export.arxiv.org/rss/cs.CE",
        ),
        (
            "Computational Geometry",
            "http://export.arxiv.org/rss/cs.CG",
        ),
        (
            "Computation and Language",
            "http://export.arxiv.org/rss/cs.CL",
        ),
        (
            "Cryptography and Security",
            "http://export.arxiv.org/rss/cs.CR",
        ),
        (
            "Computer Vision and Pattern Recognition",
            "http://export.arxiv.org/rss/cs.CV",
        ),
        ("Computers and Society", "http://export.arxiv.org/rss/cs.CY"),
        ("Databases", "http://export.arxiv.org/rss/cs.DB"),
        (
            "Distributed, Parallel, and Cluster Computing",
            "http://export.arxiv.org/rss/cs.DC",
        ),
        ("Digital Libraries", "http://export.arxiv.org/rss/cs.DL"),
        ("Discrete Mathematics", "http://export.arxiv.org/rss/cs.DM"),
        (
            "Data Structures and Algorithms",
            "http://export.arxiv.org/rss/cs.DS",
        ),
        ("Emerging Technologies", "http://export.arxiv.org/rss/cs.ET"),
        (
            "Formal Languages and Automata Theory",
            "http://export.arxiv.org/rss/cs.FL",
        ),
        ("General Literature", "http://export.arxiv.org/rss/cs.GL"),
        ("Graphics", "http://export.arxiv.org/rss/cs.GR"),
        (
            "Computer Science and Game Theory",
            "http://export.arxiv.org/rss/cs.GT",
        ),
        (
            "Human-Computer Interaction",
            "http://export.arxiv.org/rss/cs.HC",
        ),
        ("Information Retrieval", "http://export.arxiv.org/rss/cs.IR"),
        ("Information Theory", "http://export.arxiv.org/rss/cs.IT"),
        ("Machine Learning", "http://export.arxiv.org/rss/cs.LG"),
        (
            "Logic in Computer Science",
            "http://export.arxiv.org/rss/cs.LO",
        ),
        ("Multiagent Systems", "http://export.arxiv.org/rss/cs.MA"),
        ("Multimedia", "http://export.arxiv.org/rss/cs.MM"),
        ("Mathematical Software", "http://export.arxiv.org/rss/cs.MS"),
        ("Numerical Analysis", "http://export.arxiv.org/rss/cs.NA"),
        (
            "Neural and Evolutionary Computing",
            "http://export.arxiv.org/rss/cs.NE",
        ),
        (
            "Networking and Internet Architecture",
            "http://export.arxiv.org/rss/cs.NI",
        ),
        (
            "Other Computer Science",
            "http://export.arxiv.org/rss/cs.OH",
        ),
        ("Operating Systems", "http://export.arxiv.org/rss/cs.OS"),
        ("Performance", "http://export.arxiv.org/rss/cs.PF"),
        ("Programming Languages", "http://export.arxiv.org/rss/cs.PL"),
        ("Robotics", "http://export.arxiv.org/rss/cs.RO"),
        ("Sound", "http://export.arxiv.org/rss/cs.SC"),
        ("Sound", "http://export.arxiv.org/rss/cs.SD"),
        ("Software Engineering", "http://export.arxiv.org/rss/cs.SE"),
        (
            "Social and Information Networks",
            "http://export.arxiv.org/rss/cs.SI",
        ),
        ("Systems and Control", "http://export.arxiv.org/rss/cs.SY"),
        //Electrical Engineering and Systems Science
        (
            "Audio and Speech Processing",
            "http://export.arxiv.org/rss/eess.AS",
        ),
        (
            "Image and Video Processing",
            "http://export.arxiv.org/rss/eess.IV",
        ),
        ("Signal Processing", "http://export.arxiv.org/rss/eess.SP"),
        ("Systems and Control", "http://export.arxiv.org/rss/eess.SY"),
        //Mathematics
        ("Commutative Algebra", "http://export.arxiv.org/rss/math.AC"),
        ("Algebraic Geometry", "http://export.arxiv.org/rss/math.AG"),
        ("Analysis of PDEs", "http://export.arxiv.org/rss/math.AP"),
        ("Algebraic Topology", "http://export.arxiv.org/rss/math.AT"),
        (
            "Classical Analysis and ODEs",
            "http://export.arxiv.org/rss/math.CA",
        ),
        ("Combinatorics", "http://export.arxiv.org/rss/math.CO"),
        ("Category Theory", "http://export.arxiv.org/rss/math.CT"),
        ("Complex Variables", "http://export.arxiv.org/rss/math.CV"),
        (
            "Differential Geometry",
            "http://export.arxiv.org/rss/math.DG",
        ),
        ("Dynamical Systems", "http://export.arxiv.org/rss/math.DS"),
        ("Functional Analysis", "http://export.arxiv.org/rss/math.FA"),
        ("General Mathematics", "http://export.arxiv.org/rss/math.GM"),
        ("General Topology", "http://export.arxiv.org/rss/math.GN"),
        ("Group Theory", "http://export.arxiv.org/rss/math.GR"),
        ("Geometric Topology", "http://export.arxiv.org/rss/math.GT"),
        (
            "History and Overview",
            "http://export.arxiv.org/rss/math.HO",
        ),
        ("Information Theory", "http://export.arxiv.org/rss/math.IT"),
        (
            "K-Theory and Homology",
            "http://export.arxiv.org/rss/math.KT",
        ),
        ("Logic", "http://export.arxiv.org/rss/math.LO"),
        ("Metric Geometry", "http://export.arxiv.org/rss/math.MG"),
        (
            "Mathematical Physics",
            "http://export.arxiv.org/rss/math.MP",
        ),
        ("Numerical Analysis", "http://export.arxiv.org/rss/math.NA"),
        ("Number Theory", "http://export.arxiv.org/rss/math.NT"),
        ("Operator Algebras", "http://export.arxiv.org/rss/math.OA"),
        (
            "Optimization and Control",
            "http://export.arxiv.org/rss/math.OC",
        ),
        ("Probability", "http://export.arxiv.org/rss/math.PR"),
        ("Quantum Algebra", "http://export.arxiv.org/rss/math.QA"),
        ("Rings and Algebras", "http://export.arxiv.org/rss/math.RA"),
        (
            "Representation Theory",
            "http://export.arxiv.org/rss/math.RT",
        ),
        ("Symplectic Geometry", "http://export.arxiv.org/rss/math.SG"),
        ("Spectral Theory", "http://export.arxiv.org/rss/math.SP"),
        ("Statistics Theory", "http://export.arxiv.org/rss/math.ST"),
        //Nonlinear Sciences
        (
            "Adaptation and Self-Organizing System",
            "http://export.arxiv.org/rss/nlin.AO",
        ),
        ("Chaotic Dynamics", "http://export.arxiv.org/rss/nlin.CD"),
        (
            "Cellular Automata and Lattice Gases",
            "http://export.arxiv.org/rss/nlin.CG",
        ),
        (
            "Pattern Formation and Solitons",
            "http://export.arxiv.org/rss/nlin.PS",
        ),
        (
            "Exactly Solvable and Integrable Systems",
            "http://export.arxiv.org/rss/nlin.SI",
        ),
        //physics
        (
            "Accelerator Physics",
            "http://export.arxiv.org/rss/physics.acc-ph",
        ),
        (
            "Atmospheric and Oceanic Physics",
            "http://export.arxiv.org/rss/physics.ao-ph",
        ),
        (
            "Applied Physics",
            "http://export.arxiv.org/rss/physics.app-ph",
        ),
        (
            "Atomic and Molecular Clusters",
            "http://export.arxiv.org/rss/physics.atm-clus",
        ),
        (
            "Atomic Physics",
            "http://export.arxiv.org/rss/physics.atom-ph",
        ),
        (
            "Biological Physics",
            "http://export.arxiv.org/rss/physics.bio-ph",
        ),
        (
            "Chemical Physics",
            "http://export.arxiv.org/rss/physics.chem-ph",
        ),
        (
            "Classical Physics",
            "http://export.arxiv.org/rss/physics.class-ph",
        ),
        (
            "Computational Physics",
            "http://export.arxiv.org/rss/physics.comp-ph",
        ),
        (
            "Data Analysis, Statistics and Probability",
            "http://export.arxiv.org/rss/physics.data-an",
        ),
        (
            "Physics Education",
            "http://export.arxiv.org/rss/physics.ed-ph",
        ),
        (
            "Fluid Dynamics",
            "http://export.arxiv.org/rss/physics.flu-dyn",
        ),
        (
            "General Physics",
            "http://export.arxiv.org/rss/physics.gen-ph",
        ),
        ("Geophysics", "http://export.arxiv.org/rss/physics.geo-ph"),
        (
            "History and Philosophy of Physics",
            "http://export.arxiv.org/rss/physics.hist-ph",
        ),
        (
            "Instrumentation and Detectors",
            "http://export.arxiv.org/rss/physics.ins-det",
        ),
        (
            "Medical Physics",
            "http://export.arxiv.org/rss/physics.med-ph",
        ),
        ("Optics", "http://export.arxiv.org/rss/physics.optics"),
        (
            "Plasma Physics",
            "http://export.arxiv.org/rss/physics.plasm-ph",
        ),
        (
            "Popular Physics",
            "http://export.arxiv.org/rss/physics.pop-ph",
        ),
        (
            "Physics and Society",
            "http://export.arxiv.org/rss/physics.soc-ph",
        ),
        (
            "Space Physics",
            "http://export.arxiv.org/rss/physics.space-ph",
        ),
        //Quantitative Biology
        ("Biomolecules", "http://export.arxiv.org/rss/q-bio.BM"),
        ("Cell Behavior", "http://export.arxiv.org/rss/q-bio.CB"),
        ("Genomics", "http://export.arxiv.org/rss/q-bio.GN"),
        ("Molecular Networks", "http://export.arxiv.org/rss/q-bio.MN"),
        (
            "Neurons and Cognition",
            "http://export.arxiv.org/rss/q-bio.NC",
        ),
        (
            "Other Quantitative Biology",
            "http://export.arxiv.org/rss/q-bio.OT",
        ),
        (
            "Populations and Evolution",
            "http://export.arxiv.org/rss/q-bio.PE",
        ),
        (
            "Quantitative Methods",
            "http://export.arxiv.org/rss/q-bio.QM",
        ),
        (
            "Subcellular Processes",
            "http://export.arxiv.org/rss/q-bio.SC",
        ),
        ("Tissues and Organ", "http://export.arxiv.org/rss/q-bio.TO"),
        //Statistics
        (
            "General Relativity and Quantum Cosmology",
            "http://export.arxiv.org/rss/gr-qc",
        ),
        (
            "High Energy Physics - Experiment",
            "http://export.arxiv.org/rss/hep-ex",
        ),
        (
            "High Energy Physics - Lattice",
            "http://export.arxiv.org/rss/hep-lat",
        ),
        (
            "High Energy Physics - Phenomenolog",
            "http://export.arxiv.org/rss/hep-ph",
        ),
        (
            "High Energy Physics - Theory",
            "http://export.arxiv.org/rss/hep-th",
        ),
        ("Quantum Physics", "http://export.arxiv.org/rss/quant-ph"),
        ("Nuclear Experiment", "http://export.arxiv.org/rss/nucl-ex"),
        ("Nuclear Theory", "http://export.arxiv.org/rss/nucl-th"),
        ("Mathematical Physic", "http://export.arxiv.org/rss/math-ph"),
    ]
    .iter()
    .cloned()
    .collect();
    arxiv_sections_links
}
