#[path = "providers_authorization/all_providers_authorization.rs"]
mod all_providers_authorization;

#[path = "parsing/reddit/parse_reddit/get_reddit_posts.rs"]
mod get_reddit_posts;

fn main() {
    all_providers_authorization::all_providers_authorization();
    let user_subreddits: Vec<&str> = vec![
        "3Dprinting",
        "3dsmax",
        "AfterEffects",
        "architecture",
        "arduino",
        "bigdata",
        "bigquery",
        "blender",
        "BlenderGuru",
        "blenderhelp",
        "chemistry",
        "Chromium",
        "classicwow",
        "ComputerEngineering",
        "cpp",
        "cpp_questions",
        "css",
        "datascience",
        "elasticsearch",
        "FlutterDev",
        "gamedev",
        "Games",
        "git",
        "github",
        "gitlab",
        "graphql",
        "GreaseMonkey",
        "grpc",
        "hardware",
        "hearthstone",
        "Houdini",
        "javascript",
        "jenkinsci",
        "kubernetes",
        "learnmachinelearning",
        "linux",
        "low_poly",
        "MachineLearning",
        "Maya",
        "MedicalGore",
        "medizzy",
        "nasa",
        "node",
        "Physics",
        "PostgreSQL",
        "proceduralgeneration",
        "ProgrammerHumor",
        "Python",
        "pytorch",
        "QuantumComputing",
        "reactjs",
        "reactnative",
        "researchchemicals",
        "rust",
        "scientificresearch",
        "space",
        "spacex",
        "stm32f4",
        "sveltejs",
        "tutorials",
        "Unity3D",
        "unity_tutorials",
        "unrealengine",
        "warcraft3",
        "wildhearthstone",
        "wow",
    ];
    get_reddit_posts::get_reddit_posts(user_subreddits);
}

/*
#[derive(Debug, Deserialize, Serialize)]
struct Origin {
    origin: String,
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.origin)
    }
}
*/
