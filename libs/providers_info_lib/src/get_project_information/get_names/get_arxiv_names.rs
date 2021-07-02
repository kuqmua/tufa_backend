pub fn get_arxiv_names() -> Vec<String> {
    let arxiv_names: Vec<&str> = [
        "astro-ph.CO", //Cosmology and Nongalactic Astrophysics
                       // "astro-ph.EP", //Earth and Planetary Astrophysics
                       // "astro-ph.GA", //Astrophysics of Galaxies
                       // "astro-ph.HE", //High Energy Astrophysical Phenomena
                       // "astro-ph.IM", //Instrumentation and Methods for Astrophysics
                       // "astro-ph.SR", //Solar and Stellar Astrophysics
                       // //Condensed Matter
                       // "cond-mat.dis-nn",    //Disordered Systems and Neural Networks
                       // "cond-mat.mes-hall",  //Mesoscale and Nanoscale Physics
                       // "cond-mat.mtrl-sci",  //Materials Science
                       // "cond-mat.other",     //Other Condensed Matter//typicly empty
                       // "cond-mat.quant-gas", //Quantum Gases
                       // "cond-mat.soft",      //Soft Condensed Matter
                       // "cond-mat.stat-mech", //Statistical Mechanics
                       // "cond-mat.str-el",    //Strongly Correlated Electrons
                       // "cond-mat.supr-con",  //Superconductivity
                       // //Computer Science
                       // "cs.AI", //Artificial Intelligence
                       // "cs.AR", //Hardware Architecture
                       // "cs.CC", //Computational Complexity
                       // "cs.CE", //Computational Engineering, Finance, and Science
                       // "cs.CG", //Computational Geometry
                       // "cs.CL", //Computation and Language
                       // "cs.CR", //Cryptography and Security
                       // "cs.CV", //Computer Vision and Pattern Recognition
                       // "cs.CY", //Computers and Society
                       // "cs.DB", //Databases
                       // "cs.DC", //Distributed, Parallel, and Cluster Computing
                       // "cs.DL", //Digital Libraries
                       // "cs.DM", //Discrete Mathematics
                       // "cs.DS", //Data Structures and Algorithms
                       // "cs.ET", //Emerging Technologies
                       // "cs.FL", //Formal Languages and Automata Theory
                       // "cs.GL", //General Literature
                       // "cs.GR", //Graphics
                       // "cs.GT", //Computer Science and Game Theory
                       // "cs.HC", //Human-Computer Interaction
                       // "cs.IR", //Information Retrieval
                       // "cs.IT", //Information Theory
                       // "cs.LG", // Machine Learning
                       // "cs.LO", //Logic in Computer Science
                       // "cs.MA", //Multiagent Systems
                       // "cs.MM", //Multimedia
                       // "cs.MS", //Mathematical Software
                       // "cs.NA", //Numerical Analysis
                       // "cs.NE", //Neural and Evolutionary Computing
                       // "cs.NI", //Networking and Internet Architecture
                       // "cs.OH", //Other Computer Science
                       // "cs.OS", //Operating Systems
                       // "cs.PF", //Performance
                       // "cs.PL", //Programming Languages
                       // "cs.RO", //Robotics
                       // "cs.SC", //Sound
                       // "cs.SD", //Sound
                       // "cs.SE", //Software Engineering
                       // "cs.SI", //Social and Information Networks
                       // "cs.SY", //Systems and Control
                       // //Electrical Engineering and Systems Science
                       // "eess.AS", //Audio and Speech Processing
                       // "eess.IV", //Image and Video Processing
                       // "eess.SP", //Signal Processing
                       // "eess.SY", //Systems and Control
                       // //Mathematics
                       // "math.AC", //Commutative Algebra
                       // "math.AG", //Algebraic Geometry
                       // "math.AP", //Analysis of PDEs
                       // "math.AT", //Algebraic Topology
                       // "math.CA", //Classical Analysis and ODEs
                       // "math.CO", //Combinatorics
                       // "math.CT", //Category Theory
                       // "math.CV", //Complex Variables
                       // "math.DG", //Differential Geometry
                       // "math.DS", //Dynamical Systems
                       // "math.FA", //Functional Analysis
                       // "math.GM", //General Mathematics
                       // "math.GN", //General Topology
                       // "math.GR", //Group Theory
                       // "math.GT", //Geometric Topology
                       // "math.HO", //History and Overview
                       // "math.IT", //Information Theory
                       // "math.KT", //K-Theory and Homology
                       // "math.LO", //Logic
                       // "math.MG", //Metric Geometry
                       // "math.MP", //Mathematical Physics
                       // "math.NA", //Numerical Analysis
                       // "math.NT", //Number Theory
                       // "math.OA", //Operator Algebras
                       // "math.OC", //Optimization and Control
                       // "math.PR", //Probability
                       // "math.QA", //Quantum Algebra
                       // "math.RA", //Rings and Algebras
                       // "math.RT", //Representation Theory
                       // "math.SG", //Symplectic Geometry
                       // "math.SP", //Spectral Theory
                       // "math.ST", //Statistics Theory
                       // //Nonlinear Sciences
                       // "nlin.AO", //Adaptation and Self-Organizing System
                       // "nlin.CD", //Chaotic Dynamics
                       // "nlin.CG", //Cellular Automata and Lattice Gases
                       // "nlin.PS", //Pattern Formation and Solitons
                       // "nlin.SI", //Exactly Solvable and Integrable Systems
                       // //physics
                       // "physics.acc-ph",   //Accelerator Physics
                       // "physics.ao-ph",    //Atmospheric and Oceanic Physics
                       // "physics.app-ph",   //Applied Physics
                       // "physics.atm-clus", //Atomic and Molecular Clusters
                       // "physics.atom-ph",  //Atomic Physics
                       // "physics.bio-ph",   //Biological Physics
                       // "physics.chem-ph",  //Chemical Physics
                       // "physics.class-ph", //Classical Physics
                       // "physics.comp-ph",  //Computational Physics
                       // "physics.data-an",  //Data Analysis, Statistics and Probability
                       // "physics.ed-ph",    //Physics Education
                       // "physics.flu-dyn",  //Fluid Dynamics
                       // "physics.gen-ph",   //General Physics
                       // "physics.geo-ph",   //Geophysics
                       // "physics.hist-ph",  //History and Philosophy of Physics
                       // "physics.ins-det",  //Instrumentation and Detectors
                       // "physics.med-ph",   //Medical Physics
                       // "physics.optics",   //Optics
                       // "physics.plasm-ph", //Plasma Physics
                       // "physics.pop-ph",   //Popular Physics
                       // "physics.soc-ph",   //Physics and Society
                       // "physics.space-ph", //Space Physics
                       // //Quantitative Biology
                       // "q-bio.BM", //Biomolecules
                       // "q-bio.CB", //Cell Behavior
                       // "q-bio.GN", //Genomics
                       // "q-bio.MN", //Molecular Networks
                       // "q-bio.NC", //Neurons and Cognition
                       // "q-bio.OT", //Other Quantitative Biology
                       // "q-bio.PE", //Populations and Evolution
                       // "q-bio.QM", //Quantitative Methods
                       // "q-bio.SC", //Subcellular Processes
                       // "q-bio.TO", //Tissues and Organ
                       // //Statistics
                       // "gr-qc",    //General Relativity and Quantum Cosmology
                       // "hep-ex",   //High Energy Physics - Experiment
                       // "hep-lat",  //High Energy Physics - Lattice
                       // "hep-ph",   //High Energy Physics - Phenomenolog
                       // "hep-th",   //High Energy Physics - Theory
                       // "quant-ph", //Quantum Physics
                       // "nucl-ex",  //Nuclear Experiment
                       // "nucl-th",  //Nuclear Theory
                       // "math-ph",  //Mathematical Physic
    ]
    .to_vec();
    //mongodb://root:rootpassword@localhost:27017
    //mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority
    let result_of_mongo_integration = mongo_integration::mongo_get_provider_link_parts::mongo_get_provider_link_parts(
        "mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",
    "testdatabase",
    "testcollection",
    "link_part",
    );
    let mut arxiv_names: Vec<String> = Vec::new();
    match result_of_mongo_integration {
        Ok(vec_of_strings) => {
            println!("nice! {:#?}", vec_of_strings);
            arxiv_names = vec_of_strings;
        }
        Err(e) => {
            println!("F {:#?}", e);
        }
    }

    // let result_of_mongo_integration = mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection(
    //     "mongodb+srv://mongodbcloudlogin:mongodbcloudpassword@tufa-mongo.y2xob.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",
    //     "testdatabase",
    //     "testcollection",
    //     "link_part",
    //      arxiv_names.clone()
    // );
    // match result_of_mongo_integration {
    //     Ok(result_flag) => {
    //         if result_flag {
    //             println!("nice!");
    //         } else {
    //             println!("not nice!");
    //         }
    //     }
    //     Err(e) => {
    //         println!("F");
    //     }
    // }
    arxiv_names
}
