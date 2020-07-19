#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivPagesLinksStruct {
    pub Astrophysics: Astrophysics,
    pub CondensedMatter: CondensedMatter,
    pub NonlinearSciences: NonlinearSciences,
    pub ComputingResearchRepository: ComputingResearchRepository,
    pub QuantitativeBiology: QuantitativeBiology,
    pub Mathematics: Mathematics,
    pub Statistics: Statistics,
    pub ElectricalEngineeringAndSystemsScience:ElectricalEngineeringAndSystemsScience,
    pub Physics: Physics,
    pub GeneralRelativityAndQuantumCosmology: String,
    pub HighEnergyPhysicsExperiment: String,
    pub HighEnergyPhysicsLattice: String,
    pub HighEnergyPhysicsPhenomenology: String,
    pub HighEnergyPhysicsTheory: String,
    pub MathematicalPhysics: String,
    pub NuclearExperiment: String,
    pub NuclearTheory: String,
    pub QuantumPhysics: String,
}

impl ArxivPagesStruct {
    pub fn new() -> Self {
        ArxivPagesStruct {
            Astrophysics: Astrophysics::new(),
            CondensedMatter: CondensedMatter::new(),
            NonlinearSciences: NonlinearSciences::new(),
            ComputingResearchRepository: ComputingResearchRepository::new(),
            QuantitativeBiology: QuantitativeBiology::new(),
            Mathematics: Mathematics::new(),
            Statistics: Statistics::new(),
            ElectricalEngineeringAndSystemsScience:ElectricalEngineeringAndSystemsScience::new(),
            Physics: Physics::new(),
            GeneralRelativityAndQuantumCosmology: "gr-qc".to_string(),
            HighEnergyPhysicsExperiment: "hep-ex".to_string(),
            HighEnergyPhysicsLattice: "hep-lat".to_string(),
            HighEnergyPhysicsPhenomenology: "hep-ph".to_string(),
            HighEnergyPhysicsTheory: "hep-th".to_string(),
            MathematicalPhysics: "math-ph".to_string(),
            NuclearExperiment: "nucl-ex".to_string(),
            NuclearTheory: "nucl-th".to_string(),
            QuantumPhysics: "quant-ph".to_string(),
        }
    }
}

pub struct Astrophysics {
    pub AstrophysicsOfGalaxies: String,
    pub CosmologyAndNongalacticAstrophysics: String,
    pub EarthAndPlanetaryAstrophysics: String,
    pub HighEnergyAstrophysicalPhenomena: String,
    pub InstrumentationAndMethodsForAstrophysics: String,
    pub SolarAndStellarAstrophysics: String,
}

impl Astrophysics {
    pub fn new() -> Self {
        Astrophysics {
            Astrophysics of Galaxies: "".to_string(),
            Cosmology and Nongalactic Astrophysics: "".to_string(),
            EarthAndPlanetaryAstrophysics: "".to_string(),
            HighEnergyAstrophysicalPhenomena: "".to_string(),
            InstrumentationAndMethodsForAstrophysics: "".to_string(),
            SolarAndStellarAstrophysics: "".to_string(), 
        }
    }
}
pub struct CondensedMatter {
    pub Disordered Systems and Neural Networks: String,
    pub Materials Science: String,
    pub Mesoscale and Nanoscale Physics: String,
    pub Other Condensed Matte: String,
    pub Quantum Gases: String,
    pub Soft Condensed Matter: String,
    pub Statistical Mechanics: String,
    pub Strongly Correlated Electrons: String,
    pub Superconductivity: String,
}

impl CondensedMatter {
    pub fn new() -> Self {
        CondensedMatter {
            Disordered Systems and Neural Networks: "".to_string(),
            Disordered Systems and Neural Networks: "".to_string(),
            Materials Science: "".to_string(),
            Mesoscale and Nanoscale Physics: "".to_string(),
            Other Condensed Matte: "".to_string(),
            Quantum Gases: "".to_string(),
            Soft Condensed Matter: "".to_string(),
            Statistical Mechanics: "".to_string(),
            Strongly Correlated Electrons: "".to_string(),
            Superconductivity: "".to_string(),
        }
    }
}

astro-ph,
    CO, EP, GA, HE, IM, SR.
cond-mat, 
    dis-nn, mes-hall, mtrl-sci, other, quant-gas, soft, stat-mech, str-el, supr-con
cs, 
    AI, AR, CC, CE, CG, CL, CR, CV, CY, DB, DC, DL, DM, DS, ET, FL, GL, GR, GT, HC, IR, IT, LG, LO, MA, MM, MS, NA, NE, NI, OH, OS, PF, PL, RO, SC, SD, SE, SI, SY.
eess, 
    AS, IV, SP, SY.
math, 
    AC, AG, AP, AT, CA, CO, CT, CV, DG, DS, FA, GM, GN, GR, GT, HO, IT, KT, LO, MG, MP, NA, NT, OA, OC, PR, QA, RA, RT, SG, SP, ST.
nlin, 
    AO, CD, CG, PS, SI.
physics,
    acc-ph, ao-ph, app-ph, atm-clus, atom-ph, bio-ph, chem-ph, class-ph, comp-ph, data-an, ed-ph, flu-dyn, gen-ph, geo-ph, hist-ph, ins-det, med-ph, optics, plasm-ph, pop-ph, soc-ph, space-ph.
q-bio, 
    BM, CB, GN, MN, NC, OT, PE, QM, SC, TO.
stat, 
    AP, CO, ME, ML, OT, TH.
gr-qc, 
    nope
hep-ex, 
    nope
hep-lat, 
    nope
hep-ph,
    nope
hep-th, 
    nope
quant-ph, 
    nope
nucl-ex, 
    nope
nucl-th, 
    nope
math-ph, 
    nope

