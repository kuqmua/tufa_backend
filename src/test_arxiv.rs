use std::collections::HashMap;
extern crate reqwest;
extern crate xml;
use std::time::Instant;
use std::fmt::Display;
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::Client;
use futures::future;
use tokio;
use std::str;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creators: Vec<Creator>,
}

impl Display for ArxivPost {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "title = {}\nlink = {}\ndescription ={}\ncreators = {:#?}\n",
            self.title, self.link, self.description, self.creators
        )
    }
}

impl ArxivPost {
    pub fn new() -> Self {
        ArxivPost {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            creators: Vec::<Creator>::new(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Creator {
    pub name: String,
    pub link: String,
}

impl Display for Creator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "name = {}\nlink = {}\n", self.name, self.link)
    }
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
pub async fn async_test_function(vec_of_links: Vec<&str>, vec_of_keys: Vec<&str>,) -> HashMap<String, Vec<ArxivPost>>{
    let time = Instant::now();
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
    println!("arxiv bodies.len() {}", bodies.len());
    println!(
            "future::join_all (in seconds) = {} ",
            time.elapsed().as_secs()
        );
    let mut count = 0;
    let mut vec_of_vec_of_strings: Vec<Vec<String>> = Vec::new();
    for b in bodies {
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let converted_string = str::from_utf8(&slice).unwrap();
                let mut reader = Reader::from_str(&converted_string);
                reader.trim_text(true);
                let mut vec_of_arxiv_rss_strings_what_contains_xmls = Vec::new();
                let mut buf = Vec::new();
                loop {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Start(ref e)) => match e.name() {
                            b"tag1" => println!(
                                "attributes values: {:?}",
                                e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                            ),
                        _ => (),
                        },
                        Ok(Event::Text(e)) => vec_of_arxiv_rss_strings_what_contains_xmls.push(e.unescape_and_decode(&reader).unwrap()),
                        Ok(Event::Eof) => break,
                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                        _ => (),
                    }
                    buf.clear();
                }
                let vec_of_string_removed_meta = remove_meta_elements_from_vec_of_arxiv_rss(vec_of_arxiv_rss_strings_what_contains_xmls);
                vec_of_vec_of_strings.push(vec_of_string_removed_meta);
                count += 1;
            }
            Err(e) => {
                count += 1;
                eprintln!("Got an error: {}", e);
            }
        }
    }
    println!("count = {}", count);
    let mut arxiv_pages_posts_hashmap_second  = HashMap::new();
    let mut key_count = 0;
    for vec_member in vec_of_vec_of_strings {
        let vec_of_arxiv_post_second: Vec<ArxivPost> = parse_arxiv_string_xml(vec_member, vec_of_keys[key_count].to_string());
        
        if vec_of_arxiv_post_second.len() != 0{
            if vec_of_arxiv_post_second[0].creators.len() != 0 {
           arxiv_pages_posts_hashmap_second.insert(vec_of_keys[key_count].to_string(), vec_of_arxiv_post_second);
        }
        else{
            println!("vec_of_arxiv_posts[0].creators.len() == 0");
        }
            }
        key_count +=1;
    }
    // println!("arxiv_pages_posts_hashmap_second done and contains {:#?} elements", arxiv_pages_posts_hashmap_second.len());
    // println!("Artificial Intelligence contains {:#?} elements", arxiv_pages_posts_hashmap_second["Artificial Intelligence"].len());
    println!("arxiv fetching/parsing done in {} seconds",time.elapsed().as_secs());
    arxiv_pages_posts_hashmap_second
}

pub fn test_arxiv() ->  HashMap<String, Vec<ArxivPost>>
{
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links_in_hash_map();
    println!("{:#?}", arxiv_links_in_hash_map.len());
    let vec_of_links: Vec<&str> = arxiv_links_in_hash_map.values().cloned().collect();
    let vec_of_keys: Vec<&str> = arxiv_links_in_hash_map.keys().cloned().collect();
    let vec_of_vec_of_strings = async_test_function(vec_of_links, vec_of_keys);
    vec_of_vec_of_strings
}

fn remove_meta_elements_from_vec_of_arxiv_rss(mut vec_of_arxiv_rss: Vec<String>)->Vec<String>{
    let mut remove_element = 0;
    while remove_element < 13 {
        vec_of_arxiv_rss.remove(0);
        remove_element += 1;
    }
    vec_of_arxiv_rss
}

pub fn parse_arxiv_string_xml(vec_of_arxiv_xml_string: Vec<String>, key: String)->Vec<ArxivPost>{
    let mut vec_of_arxiv_posts: Vec<ArxivPost> = Vec::new();
    let mut posts_count = 0;
    let end_index_title_string = ". (arXiv";
    let start_index_description_string = "<p>";
    let end_index_description_string = "\n</p>";
    let start_index_creator_link_string = "<a href=\"";
    let end_index_creator_link_string = "\">";
    let end_index_creator_name_string = "</a>";
    while posts_count < vec_of_arxiv_xml_string.len() {
        let mut arxiv_post: ArxivPost = ArxivPost::new();
        let start_title = vec_of_arxiv_xml_string[posts_count].clone();
        if let Some(index) = start_title.find(end_index_title_string) {
            arxiv_post.title = start_title[..index].to_string();
        } else {
            arxiv_post.title = start_title;
            println!("whole title written and start.find(. (arXiv) works uncorrectly");
        }
        arxiv_post.link = vec_of_arxiv_xml_string[posts_count + 1].clone();
        let description_start = vec_of_arxiv_xml_string[posts_count + 2].clone();
        if let Some(index_from_start) = description_start.find(start_index_description_string) {
            if let Some(index_from_end) = description_start.find(end_index_description_string) {
                arxiv_post.description =
                    description_start[index_from_start + start_index_description_string.len()..index_from_end].to_string();
            } else {
                arxiv_post.description =
                    description_start[index_from_start + start_index_description_string.len()..].to_string();
                println!(
                    "error to find index_from_end, but index_from_start was found, posts_count={}",
                    posts_count
                );
            }
        } else {
            if let Some(index_from_end) = description_start.find(end_index_description_string) {
                arxiv_post.description = description_start[..index_from_end].to_string();
                println!(
                    "error to find index_from_start, but index_from_end was found, posts_count={}",
                    posts_count
                );
            } else {
                arxiv_post.description = description_start;
                println!("whole title written and description_start.find() worked uncorrectly, posts_count={}", posts_count);
            }
        }
        let creators_string = vec_of_arxiv_xml_string[posts_count + 3].clone();
        let mut string_part_for_loop = creators_string;
        let mut creators_count = 0;
        while let Some(link_index_from_start) = string_part_for_loop.find(start_index_creator_link_string) {
            if creators_count > 256 {
                panic!("why creators_count > 256?"); //переписать когда примешь стандарт по которому ошибки обрабатывать будешь
            } else {
                if let Some(link_index_from_end) = string_part_for_loop.find(end_index_creator_link_string) {
                    if let Some(name_index_from_end) = string_part_for_loop.find(end_index_creator_name_string) {
                        let mut creator = Creator::new();
                        creator.link = string_part_for_loop[link_index_from_start + start_index_creator_link_string.len()..link_index_from_end].to_string();
                        let name_index_from_start = link_index_from_end + end_index_creator_link_string.len();
                        creator.name = string_part_for_loop[name_index_from_start..name_index_from_end].to_string();
                        arxiv_post.creators.push(creator);
                        string_part_for_loop = string_part_for_loop[name_index_from_end + end_index_creator_name_string.len()..].to_string();
                        creators_count += 1;
                    } else {
                        let mut creator = Creator::new();
                        creator.link = string_part_for_loop[link_index_from_start + start_index_creator_link_string.len()..link_index_from_end].to_string();
                        creators_count += 1;
                        println!(
                    "error to find name_index_from_end, but name_index_from_start(what equals to link_index_from_end + its len) was found, Creator pushed with default creator.name, creators_count={}",
                    creators_count
                );
                    }
                } else {
                    creators_count += 1;
                    println!(
                    "error to find link_index_from_end, but link_index_from_start was found, creators_count={}",
                    creators_count      
                );
                break
                }
            }
        }
        vec_of_arxiv_posts.push(arxiv_post);
        posts_count += 4;
    }
    
    // if vec_of_arxiv_posts.len() == 0{
    //     println!("0 elements in {}",key); 
    // }
    // else if vec_of_arxiv_posts[0].creators.len() == 0{         
    //         println!("0 creators in {}", key);
    // }
    // else {
    //     println!("{} elements in key {}", posts_count,  key);
    // }
    vec_of_arxiv_posts
}

pub fn get_arxiv_links_in_hash_map() -> HashMap<&'static str, &'static str> {
    let arxiv_sections_links: HashMap<&str,&str> =
    //Astrophysics
    [("Cosmology and Nongalactic Astrophysics","http://export.arxiv.org/rss/astro-ph.CO"),
     ("Earth and Planetary Astrophysics","http://export.arxiv.org/rss/astro-ph.EP"),
     ("Astrophysics of Galaxies","http://export.arxiv.org/rss/astro-ph.GA"),
     ("High Energy Astrophysical Phenomena","http://export.arxiv.org/rss/astro-ph.HE"),
     ("Instrumentation and Methods for Astrophysics","http://export.arxiv.org/rss/astro-ph.IM"),
     ("Solar and Stellar Astrophysics","http://export.arxiv.org/rss/astro-ph.SR"),
     //Condensed Matter
     ("Disordered Systems and Neural Networks","http://export.arxiv.org/rss/cond-mat.dis-nn"),
     ("Mesoscale and Nanoscale Physics","http://export.arxiv.org/rss/cond-mat.mes-hall"),
     ("Materials Science","http://export.arxiv.org/rss/cond-mat.mtrl-sci"),
     ("Other Condensed Matter","http://export.arxiv.org/rss/cond-mat.other"),//НАДО ЛИ - УПАДЕТ ЖЕ 
     ("Quantum Gases","http://export.arxiv.org/rss/cond-mat.quant-gas"),
     ("Soft Condensed Matter","http://export.arxiv.org/rss/cond-mat.soft"),
     ("Statistical Mechanics","http://export.arxiv.org/rss/cond-mat.stat-mech"),
     ("Strongly Correlated Electrons","http://export.arxiv.org/rss/cond-mat.str-el"),
     ("Superconductivity","http://export.arxiv.org/rss/cond-mat.supr-con"),
     //Computer Science
     ("Artificial Intelligence","http://export.arxiv.org/rss/cs.AI"),
     ("Hardware Architecture","http://export.arxiv.org/rss/cs.AR"),
     ("Computational Complexity","http://export.arxiv.org/rss/cs.CC"),
     ("Computational Engineering, Finance, and Science","http://export.arxiv.org/rss/cs.CE"),
     ("Computational Geometry","http://export.arxiv.org/rss/cs.CG"),
     ("Computation and Language","http://export.arxiv.org/rss/cs.CL"),
     ("Cryptography and Security","http://export.arxiv.org/rss/cs.CR"),
     ("Computer Vision and Pattern Recognition","http://export.arxiv.org/rss/cs.CV"),
     ("Computers and Society","http://export.arxiv.org/rss/cs.CY"),
     ("Databases","http://export.arxiv.org/rss/cs.DB"),
     ("Distributed, Parallel, and Cluster Computing","http://export.arxiv.org/rss/cs.DC"),
     ("Digital Libraries","http://export.arxiv.org/rss/cs.DL"),
     ("Discrete Mathematics","http://export.arxiv.org/rss/cs.DM"),
     ("Data Structures and Algorithms","http://export.arxiv.org/rss/cs.DS"),
     ("Emerging Technologies","http://export.arxiv.org/rss/cs.ET"),
     ("Formal Languages and Automata Theory","http://export.arxiv.org/rss/cs.FL"),
     ("General Literature","http://export.arxiv.org/rss/cs.GL"),
     ("Graphics","http://export.arxiv.org/rss/cs.GR"),
     ("Computer Science and Game Theory","http://export.arxiv.org/rss/cs.GT"),
     ("Human-Computer Interaction","http://export.arxiv.org/rss/cs.HC"),
     ("Information Retrieval","http://export.arxiv.org/rss/cs.IR"),
     ("Information Theory","http://export.arxiv.org/rss/cs.IT"),
     ("Machine Learning","http://export.arxiv.org/rss/cs.LG"),
     ("Logic in Computer Science","http://export.arxiv.org/rss/cs.LO"),
     ("Multiagent Systems","http://export.arxiv.org/rss/cs.MA"),
     ("Multimedia","http://export.arxiv.org/rss/cs.MM"),
     ("Mathematical Software","http://export.arxiv.org/rss/cs.MS"),
     ("Numerical Analysis","http://export.arxiv.org/rss/cs.NA"),
     ("Neural and Evolutionary Computing","http://export.arxiv.org/rss/cs.NE"),
     ("Networking and Internet Architecture","http://export.arxiv.org/rss/cs.NI"),
     ("Other Computer Science","http://export.arxiv.org/rss/cs.OH"),
     ("Operating Systems","http://export.arxiv.org/rss/cs.OS"),
     ("Performance","http://export.arxiv.org/rss/cs.PF"),
     ("Programming Languages","http://export.arxiv.org/rss/cs.PL"),
     ("Robotics","http://export.arxiv.org/rss/cs.RO"),
     ("Sound","http://export.arxiv.org/rss/cs.SC"),
     ("Sound","http://export.arxiv.org/rss/cs.SD"),
     ("Software Engineering","http://export.arxiv.org/rss/cs.SE"),
     ("Social and Information Networks","http://export.arxiv.org/rss/cs.SI"),
     ("Systems and Control","http://export.arxiv.org/rss/cs.SY"),
     //Electrical Engineering and Systems Science
     ("Audio and Speech Processing","http://export.arxiv.org/rss/eess.AS"),
     ("Image and Video Processing","http://export.arxiv.org/rss/eess.IV"),
     ("Signal Processing","http://export.arxiv.org/rss/eess.SP"),
     ("Systems and Control","http://export.arxiv.org/rss/eess.SY"),
     //Mathematics
     ("Commutative Algebra","http://export.arxiv.org/rss/math.AC"),
     ("Algebraic Geometry","http://export.arxiv.org/rss/math.AG"),
     ("Analysis of PDEs","http://export.arxiv.org/rss/math.AP"),
     ("Algebraic Topology","http://export.arxiv.org/rss/math.AT"),
     ("Classical Analysis and ODEs","http://export.arxiv.org/rss/math.CA"),
     ("Combinatorics","http://export.arxiv.org/rss/math.CO"),
     ("Category Theory","http://export.arxiv.org/rss/math.CT"),
     ("Complex Variables","http://export.arxiv.org/rss/math.CV"),
     ("Differential Geometry","http://export.arxiv.org/rss/math.DG"),
     ("Dynamical Systems","http://export.arxiv.org/rss/math.DS"),
     ("Functional Analysis","http://export.arxiv.org/rss/math.FA"),
     ("General Mathematics","http://export.arxiv.org/rss/math.GM"),
     ("General Topology","http://export.arxiv.org/rss/math.GN"),
     ("Group Theory","http://export.arxiv.org/rss/math.GR"),
     ("Geometric Topology","http://export.arxiv.org/rss/math.GT"),
     ("History and Overview","http://export.arxiv.org/rss/math.HO"),
     ("Information Theory","http://export.arxiv.org/rss/math.IT"),
     ("K-Theory and Homology","http://export.arxiv.org/rss/math.KT"),
     ("Logic","http://export.arxiv.org/rss/math.LO"),
     ("Metric Geometry","http://export.arxiv.org/rss/math.MG"),
     ("Mathematical Physics","http://export.arxiv.org/rss/math.MP"),
     ("Numerical Analysis","http://export.arxiv.org/rss/math.NA"),
     ("Number Theory","http://export.arxiv.org/rss/math.NT"),
     ("Operator Algebras","http://export.arxiv.org/rss/math.OA"),
     ("Optimization and Control","http://export.arxiv.org/rss/math.OC"),
     ("Probability","http://export.arxiv.org/rss/math.PR"),
     ("Quantum Algebra","http://export.arxiv.org/rss/math.QA"),
     ("Rings and Algebras","http://export.arxiv.org/rss/math.RA"),
     ("Representation Theory","http://export.arxiv.org/rss/math.RT"),
     ("Symplectic Geometry","http://export.arxiv.org/rss/math.SG"),
     ("Spectral Theory","http://export.arxiv.org/rss/math.SP"),
     ("Statistics Theory","http://export.arxiv.org/rss/math.ST"),
     //Nonlinear Sciences
     ("Adaptation and Self-Organizing System","http://export.arxiv.org/rss/nlin.AO"),
     ("Chaotic Dynamics","http://export.arxiv.org/rss/nlin.CD"),
     ("Cellular Automata and Lattice Gases","http://export.arxiv.org/rss/nlin.CG"),
     ("Pattern Formation and Solitons","http://export.arxiv.org/rss/nlin.PS"),
     ("Exactly Solvable and Integrable Systems","http://export.arxiv.org/rss/nlin.SI"),
     //physics
     ("Accelerator Physics","http://export.arxiv.org/rss/physics.acc-ph"),
     ("Atmospheric and Oceanic Physics","http://export.arxiv.org/rss/physics.ao-ph"),
     ("Applied Physics","http://export.arxiv.org/rss/physics.app-ph"),
     ("Atomic and Molecular Clusters","http://export.arxiv.org/rss/physics.atm-clus"),
     ("Atomic Physics","http://export.arxiv.org/rss/physics.atom-ph"),
     ("Biological Physics","http://export.arxiv.org/rss/physics.bio-ph"),
     ("Chemical Physics","http://export.arxiv.org/rss/physics.chem-ph"),
     ("Classical Physics","http://export.arxiv.org/rss/physics.class-ph"),
     ("Computational Physics","http://export.arxiv.org/rss/physics.comp-ph"),
     ("Data Analysis, Statistics and Probability","http://export.arxiv.org/rss/physics.data-an"),
     ("Physics Education","http://export.arxiv.org/rss/physics.ed-ph"),
     ("Fluid Dynamics","http://export.arxiv.org/rss/physics.flu-dyn"),
     ("General Physics","http://export.arxiv.org/rss/physics.gen-ph"),
     ("Geophysics","http://export.arxiv.org/rss/physics.geo-ph"),
     ("History and Philosophy of Physics","http://export.arxiv.org/rss/physics.hist-ph"),
     ("Instrumentation and Detectors","http://export.arxiv.org/rss/physics.ins-det"),
     ("Medical Physics","http://export.arxiv.org/rss/physics.med-ph"),
     ("Optics","http://export.arxiv.org/rss/physics.optics"),
     ("Plasma Physics","http://export.arxiv.org/rss/physics.plasm-ph"),
     ("Popular Physics","http://export.arxiv.org/rss/physics.pop-ph"),
     ("Physics and Society","http://export.arxiv.org/rss/physics.soc-ph"),
     ("Space Physics","http://export.arxiv.org/rss/physics.space-ph"),
     //Quantitative Biology
     ("Biomolecules","http://export.arxiv.org/rss/q-bio.BM"),
     ("Cell Behavior","http://export.arxiv.org/rss/q-bio.CB"),
     ("Genomics","http://export.arxiv.org/rss/q-bio.GN"),
     ("Molecular Networks","http://export.arxiv.org/rss/q-bio.MN"),
     ("Neurons and Cognition","http://export.arxiv.org/rss/q-bio.NC"),
     ("Other Quantitative Biology","http://export.arxiv.org/rss/q-bio.OT"),
     ("Populations and Evolution","http://export.arxiv.org/rss/q-bio.PE"),
     ("Quantitative Methods","http://export.arxiv.org/rss/q-bio.QM"),
     ("Subcellular Processes","http://export.arxiv.org/rss/q-bio.SC"),
     ("Tissues and Organ","http://export.arxiv.org/rss/q-bio.TO"),
     //Statistics
     ("General Relativity and Quantum Cosmology","http://export.arxiv.org/rss/gr-qc"),
     ("High Energy Physics - Experiment","http://export.arxiv.org/rss/hep-ex"),
     ("High Energy Physics - Lattice","http://export.arxiv.org/rss/hep-lat"),
     ("High Energy Physics - Phenomenolog","http://export.arxiv.org/rss/hep-ph"),
     ("High Energy Physics - Theory","http://export.arxiv.org/rss/hep-th"),
     ("Quantum Physics","http://export.arxiv.org/rss/quant-ph"),
     ("Nuclear Experiment","http://export.arxiv.org/rss/nucl-ex"),
     ("Nuclear Theory","http://export.arxiv.org/rss/nucl-th"),
     ("Mathematical Physic","http://export.arxiv.org/rss/math-ph"),
     ]
     .iter().cloned().collect();
    arxiv_sections_links
}