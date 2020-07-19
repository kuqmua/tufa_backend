use std::collections::HashMap;


pub fn get_arxiv_pages() -> Vec<&'static str> {
    //let mut arxiv_sections_links: HashMap<&str, &str> = HashMap::new();
    
    let arxiv_sections_links: HashMap<&str,&str> =
    //Astrophysics
    [("Cosmology and Nongalactic Astrophysics", "http://export.arxiv.org/rss/astro-ph.CO"),
     ("Earth and Planetary Astrophysics", "http://export.arxiv.org/rss/astro-ph.EP"),
     ("Astrophysics of Galaxies", "http://export.arxiv.org/rss/astro-ph.GA")
     ("High Energy Astrophysical Phenomena", "http://export.arxiv.org/rss/astro-ph.HE")
     ("Instrumentation and Methods for Astrophysics", "http://export.arxiv.org/rss/astro-ph.IM")
     ("Solar and Stellar Astrophysics", "http://export.arxiv.org/rss/astro-ph.SR")
     //Condensed Matter
     ("Disordered Systems and Neural Networks", "http://export.arxiv.org/rss/cond-mat.dis-nn")
     ("Mesoscale and Nanoscale Physics", "http://export.arxiv.org/rss/cond-mat.mes-hall")
     ("Materials Science", "http://export.arxiv.org/rss/cond-mat.mtrl-sci")
     ("Other Condensed Matter", "http://export.arxiv.org/rss/cond-mat.other")//НАДО ЛИ - УПАДЕТ ЖЕ 
     ("Quantum Gases", "http://export.arxiv.org/rss/cond-mat.quant-gas")
     ("Soft Condensed Matter", "http://export.arxiv.org/rss/cond-mat.soft")
     ("Statistical Mechanics", "http://export.arxiv.org/rss/cond-mat.stat-mech")
     ("Strongly Correlated Electrons", "http://export.arxiv.org/rss/cond-mat.str-el")
     ("Superconductivity", "http://export.arxiv.org/rss/cond-mat.supr-con")
     //Computer Science
     ("Artificial Intelligence", "http://export.arxiv.org/rss/cs.AI")
     ("Hardware Architecture", "http://export.arxiv.org/rss/cs.AR")
     ("Computational Complexity", "http://export.arxiv.org/rss/cs.CC")
     ("Computational Engineering, Finance, and Science", "http://export.arxiv.org/rss/cs.CE")
     ("Computational Geometry", "http://export.arxiv.org/rss/cs.CG")
     ("Computation and Language", "http://export.arxiv.org/rss/cs.CL")
     ("Cryptography and Security", "http://export.arxiv.org/rss/cs.CR")
     ("Computer Vision and Pattern Recognition", "http://export.arxiv.org/rss/cs.CV")
     ("Computers and Society", "http://export.arxiv.org/rss/cs.CY")
     ("Databases", "http://export.arxiv.org/rss/cs.DB")
     ("Distributed, Parallel, and Cluster Computing", "http://export.arxiv.org/rss/cs.DC")
     ("Digital Libraries", "http://export.arxiv.org/rss/cs.DL")
     ("Discrete Mathematics", "http://export.arxiv.org/rss/cs.DM")
     ("Data Structures and Algorithms", "http://export.arxiv.org/rss/cs.DS")
     ("Emerging Technologies", "http://export.arxiv.org/rss/cs.ET")
     ("Formal Languages and Automata Theory", "http://export.arxiv.org/rss/cs.FL")
     ("General Literature", "http://export.arxiv.org/rss/cs.GL")
     ("Graphics", "http://export.arxiv.org/rss/cs.GR")
     ("Computer Science and Game Theory", "http://export.arxiv.org/rss/cs.GT")
     ("Human-Computer Interaction", "http://export.arxiv.org/rss/cs.HC")
     ("Information Retrieval", "http://export.arxiv.org/rss/cs.IR")
     ("Information Theory", "http://export.arxiv.org/rss/cs.IT")
     ("Machine Learning", "http://export.arxiv.org/rss/cs.LG")
     ("Logic in Computer Science", "http://export.arxiv.org/rss/cs.LO")
     ("Multiagent Systems", "http://export.arxiv.org/rss/cs.MA")
     ("Multimedia", "http://export.arxiv.org/rss/cs.MM")
     ("Mathematical Software", "http://export.arxiv.org/rss/cs.MS")
     ("Numerical Analysis", "http://export.arxiv.org/rss/cs.NA")
     ("Neural and Evolutionary Computing", "http://export.arxiv.org/rss/cs.NE")
     ("Networking and Internet Architecture", "http://export.arxiv.org/rss/cs.NI")
     ("Other Computer Science", "http://export.arxiv.org/rss/cs.OH")
     ("Operating Systems", "http://export.arxiv.org/rss/cs.OS")
     ("Performance", "http://export.arxiv.org/rss/cs.PF")
     ("Programming Languages", "http://export.arxiv.org/rss/cs.PL")
     ("Robotics", "http://export.arxiv.org/rss/cs.RO")
     ("Sound", "http://export.arxiv.org/rss/cs.SC")
     ("Sound", "http://export.arxiv.org/rss/cs.SD")
     ("Software Engineering", "http://export.arxiv.org/rss/cs.SE")
     ("Social and Information Networks", "http://export.arxiv.org/rss/cs.SI")
     ("Systems and Control", "http://export.arxiv.org/rss/cs.SY")
     //Electrical Engineering and Systems Science
     ("Audio and Speech Processing", "http://export.arxiv.org/rss/eess.AS")
     ("Image and Video Processing", "http://export.arxiv.org/rss/eess.IV")
     ("Signal Processing", "http://export.arxiv.org/rss/eess.SP")
     ("Systems and Control", "http://export.arxiv.org/rss/eess.SY")
     //Mathematics
     ("Commutative Algebra", "http://export.arxiv.org/rss/math.AC")
     ("Algebraic Geometry", "http://export.arxiv.org/rss/math.AG")
     ("Analysis of PDEs", "http://export.arxiv.org/rss/math.AP")
     ("Algebraic Topology", "http://export.arxiv.org/rss/math.AT")
     ("Classical Analysis and ODEs", "http://export.arxiv.org/rss/math.CA")
     ("Combinatorics", "http://export.arxiv.org/rss/math.CO")
     ("Category Theory", "http://export.arxiv.org/rss/math.CT")
     ("Complex Variables", "http://export.arxiv.org/rss/math.CV")
     ("Differential Geometry", "http://export.arxiv.org/rss/math.DG")
     ("Dynamical Systems", "http://export.arxiv.org/rss/math.DS")
     ("Functional Analysis", "http://export.arxiv.org/rss/math.FA")
     ("General Mathematics", "http://export.arxiv.org/rss/math.GM")
     ("General Topology", "http://export.arxiv.org/rss/math.GN")
     ("Group Theory", "http://export.arxiv.org/rss/math.GR")
     ("Geometric Topology", "http://export.arxiv.org/rss/math.GT")
     ("History and Overview", "http://export.arxiv.org/rss/math.HO")
     ("Information Theory", "http://export.arxiv.org/rss/math.IT")
     ("K-Theory and Homology", "http://export.arxiv.org/rss/math.KT")
     ("Logic", "http://export.arxiv.org/rss/math.LO")
     ("Metric Geometry", "http://export.arxiv.org/rss/math.MG")
     ("Mathematical Physics", "http://export.arxiv.org/rss/math.MP")
     ("Numerical Analysis", "http://export.arxiv.org/rss/math.NA")
     ("Number Theory", "http://export.arxiv.org/rss/math.NT")
     ("Operator Algebras", "http://export.arxiv.org/rss/math.OA")
     ("Optimization and Control", "http://export.arxiv.org/rss/math.OC")
     ("Probability", "http://export.arxiv.org/rss/math.PR")
     ("Quantum Algebra", "http://export.arxiv.org/rss/math.QA")
     ("Rings and Algebras", "http://export.arxiv.org/rss/math.RA")
     ("Representation Theory", "http://export.arxiv.org/rss/math.RT")
     ("Symplectic Geometry", "http://export.arxiv.org/rss/math.SG")
     ("Spectral Theory", "http://export.arxiv.org/rss/math.SP")
     ("Statistics Theory", "http://export.arxiv.org/rss/math.ST")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ("", "")
     ]
     .iter().cloned().collect();
    



   
    
    ////////////

    Astrophysic.insert("Astrophysics", "astro-ph");
    Astrophysic.insert("Condensed Matter", "cond-mat");
    Astrophysic.insert("General Relativity and Quantum Cosmology", "gr-qc");
    Astrophysic.insert("High Energy Physics - Experiment", "hep-ex");
    Astrophysic.insert("High Energy Physics - Lattice", "hep-lat");
    Astrophysic.insert("High Energy Physics - Phenomenology", "hep-ph");
    Astrophysic.insert("High Energy Physics - Theory", "hep-th");
    Astrophysic.insert("Mathematical Physics", "math-ph");
    Astrophysic.insert("Nonlinear Sciences", "nlin");
    Astrophysic.insert("Nuclear Experiment", "nucl-ex");
    Astrophysic.insert("Nuclear Theory", "nucl-th");
    Astrophysic.insert("Physics", "physics");
    Astrophysic.insert("Quantum Physics", "quant-ph");
    Astrophysic.insert("Mathematics", "math");
    Astrophysic.insert("Computing Research Repository", "cs");//
    Astrophysic.insert("Quantitative Biology", "q-bio");
    Astrophysic.insert("Statistics", "stat");
    Astrophysic.insert("Electrical Engineering and Systems Science", "eess");



    let Astrophysics: Vec<&str> = vec![
        "Astrophysics of Galaxies",
        "Cosmology and Nongalactic Astrophysics",
        "Earth and Planetary Astrophysics",
        "High Energy Astrophysical Phenomena",
        "Instrumentation and Methods for Astrophysics",
        "Solar and Stellar Astrophysics",
    ];
    let CondensedMatter: Vec<&str> = vec![
        "Disordered Systems and Neural Networks",
        "Materials Science",
        "Mesoscale and Nanoscale Physics",
        "Other Condensed Matter",
        "Quantum Gases",
        "Soft Condensed Matter",
        "Statistical Mechanics",
        "Strongly Correlated Electrons",
        "Superconductivity",
    ];
    let Nonlinear Sciences: Vec<&str> = vec![
        "Adaptation and Self-Organizing Systems",
        "Cellular Automata and Lattice Gases",
        "Chaotic Dynamics",
        "Exactly Solvable and Integrable Systems",
        "Pattern Formation and Solitons",
    ];
    let Physics: Vec<&str> = vec![
        "Accelerator Physics",
        "Applied Physics",
        "Atmospheric and Oceanic Physics",
        "Atomic and Molecular Clusters",
        "Atomic Physics",
        "Biological Physics",
        "Chemical Physics",
        "Classical Physics",
        "Computational Physics",
        "Data Analysis",
        "Statistics and Probability",
        "Fluid Dynamics",
        "General Physics",
        "Geophysics",
        "History and Philosophy of Physics",
        "History and Philosophy of Physics",
        "Medical Physics",
        "Optics",
        "Physics and Society",
        "Physics Education",
        "Plasma Physics",
        "Popular Physics",
        "Space Physics",
         ];
    let Mathematics: Vec<&str> = vec![
        "Algebraic Geometry",
        "Algebraic Topology",
        "Analysis of PDEs",
        "Category Theory",
        "Classical Analysis and ODEs",
        "Combinatorics",
        "Commutative Algebra",
        "Complex Variables",
        "Differential Geometry",
        "Dynamical Systems",
        "Functional Analysis",
        "General Mathematics",
        "General Topology",
        "Geometric Topology",
        "Group Theory",
        "History and Overview",
        "Information Theory",
        "K-Theory and Homology",
        "Logic",
        "Mathematical Physics",
        "Metric Geometry",
        "Number Theory",
        "Numerical Analysis",
        "Operator Algebras",
        "Optimization and Control",
        "Probability",
        "Quantum Algebra",
        "Representation Theory",
        "Rings and Algebras",
        "Spectral Theory",
        "Statistics Theory",
        "Symplectic Geometry",
    ];
    let Computing Research Repository: Vec<&str> = vec![
        "Artificial Intelligence",
        "Computation and Language",
        "Computational Complexity",
        "Computational Engineering",
        "Finance, and Science",
        "Computational Geometry",
        "Computer Science and Game Theory",
        "Computer Vision and Pattern Recognition",
        "Computers and Society",
        "Cryptography and Security",
        "Data Structures and Algorithms",
        "Databases",
        "Digital Libraries",
        "Discrete Mathematics",
        "Distributed, Parallel, and Cluster Computing",
        "Emerging Technologies",
        "Formal Languages and Automata Theory",
        "General Literature",
        "Graphics",
        "Hardware Architecture",
        "Human-Computer Interaction",
        "Information Retrieval",
        "Information Theory",
        "Logic in Computer Science",
        "Machine Learning",
        "Mathematical Software",
        "Multiagent Systems",
        "Multimedia",
        "Networking and Internet Architecture",
        "Neural and Evolutionary Computing",
        "Numerical Analysis",
        "Operating Systems",
        "Other Computer Science",
        "Performance",
        "Programming Languages",
        "Robotics",
        "Social and Information Networks",
        "Software Engineering",
        "Sound",
        "Symbolic Computation",
        "Systems and Control",
    ];
    let Quantitative Biology: Vec<&str> = vec![
        "Biomolecules",
        "Cell Behavior",
        "Genomics",
        "Molecular Networks",
        "Neurons and Cognition",
        "Other Quantitative Biology",
        "Populations and Evolution",
        "Quantitative Methods",
        "Subcellular Processes",
        "Tissues and Organs",
    ];
    let Statistics: Vec<&str> = vec![
        "Applications",
        "Computation",
        "Machine Learning",
        "Methodology",
        "Other Statistics",
        "Statistics Theory",
    ];
    let Electrical Engineering and Systems Science: Vec<&str> = vec![
        "Audio and Speech Processing",
        "Image and Video Processing",
        "Signal Processing",
        "Systems and Control",
    ];
    let GeneralRelativityAndQuantumCosmology = "";
    let HighEnergyPhysicsExperiment = "";
    let HighEnergyPhysicsLattice = "";
    let HighEnergyPhysicsPhenomenology = "";
    let HighEnergyPhysicsTheory = "";
    let MathematicalPhysics = "";
    let NuclearExperiment = "";
    let NuclearTheory = "";
    let QuantumPhysics = "";
    arxiv_sections
}
