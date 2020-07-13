/*

#[path = "parsing/reddit/parse_reddit/get_reddit_posts.rs"]
mod get_reddit_posts;

fn main() {
    let vec_reddit_posts = get_reddit_posts::get_reddit_posts(subreddits_names);
    println!("{}", vec_reddit_posts[0])
}
*/

#[path = "providers_authorization/all_providers_authorization.rs"]
mod all_providers_authorization;

#[path = "parsing/reddit/parse_reddit/get_posts.rs"]
mod get_posts;
use get_posts::get_posts;
fn main() {
    let subreddits_names: Vec<&str> = vec![
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
    all_providers_authorization::all_providers_authorization();
    let vec = get_posts(subreddits_names);
    println!("{:#?}", vec[0].author);
}
/*

*/
/*
let urls = vec![
        "https://www.reddit.com/r/3Dprinting/new.json",
        "https://www.reddit.com/r/3dsmax/new.json",
        "https://www.reddit.com/r/AfterEffects/new.json",
        "https://www.reddit.com/r/architecture/new.json",
        "https://www.reddit.com/r/arduino/new.json",
        "https://www.reddit.com/r/bigdata/new.json",
        "https://www.reddit.com/r/bigquery/new.json",
        "https://www.reddit.com/r/blender/new.json",
        "https://www.reddit.com/r/BlenderGuru/new.json",
        "https://www.reddit.com/r/blenderhelp/new.json",
        "https://www.reddit.com/r/chemistry/new.json",
        "https://www.reddit.com/r/Chromium/new.json",
        "https://www.reddit.com/r/classicwow/new.json",
        "https://www.reddit.com/r/ComputerEngineering/new.json",
        "https://www.reddit.com/r/cpp/new.json",
        "https://www.reddit.com/r/cpp_questions/new.json",
        "https://www.reddit.com/r/css/new.json",
        "https://www.reddit.com/r/datascience/new.json",
        "https://www.reddit.com/r/elasticsearch/new.json",
        "https://www.reddit.com/r/FlutterDev/new.json",
        "https://www.reddit.com/r/gamedev/new.json",
        "https://www.reddit.com/r/Games/new.json",
        "https://www.reddit.com/r/git/new.json",
        "https://www.reddit.com/r/github/new.json",
        "https://www.reddit.com/r/gitlab/new.json",
        "https://www.reddit.com/r/graphql/new.json",
        "https://www.reddit.com/r/GreaseMonkey/new.json",
        "https://www.reddit.com/r/grpc/new.json",
        "https://www.reddit.com/r/hardware/new.json",
        "https://www.reddit.com/r/hearthstone/new.json",
        "https://www.reddit.com/r/Houdini/new.json",
        "https://www.reddit.com/r/javascript/new.json",
        "https://www.reddit.com/r/jenkinsci/new.json",
        "https://www.reddit.com/r/kubernetes/new.json",
        "https://www.reddit.com/r/learnmachinelearning/new.json",
        "https://www.reddit.com/r/linux/new.json",
        "https://www.reddit.com/r/low_poly/new.json",
        "https://www.reddit.com/r/MachineLearning/new.json",
        "https://www.reddit.com/r/Maya/new.json",
        "https://www.reddit.com/r/MedicalGore/new.json",
        "https://www.reddit.com/r/medizzy/new.json",
        "https://www.reddit.com/r/nasa/new.json",
        "https://www.reddit.com/r/node/new.json",
        "https://www.reddit.com/r/Physics/new.json",
        "https://www.reddit.com/r/PostgreSQL/new.json",
        "https://www.reddit.com/r/proceduralgeneration/new.json",
        "https://www.reddit.com/r/ProgrammerHumor/new.json",
        "https://www.reddit.com/r/Python/new.json",
        "https://www.reddit.com/r/pytorch/new.json",
        "https://www.reddit.com/r/QuantumComputing/new.json",
        "https://www.reddit.com/r/reactjs/new.json",
        "https://www.reddit.com/r/reactnative/new.json",
        "https://www.reddit.com/r/researchchemicals/new.json",
        "https://www.reddit.com/r/rust/new.json",
        "https://www.reddit.com/r/scientificresearch/new.json",
        "https://www.reddit.com/r/space/new.json",
        "https://www.reddit.com/r/spacex/new.json",
        "https://www.reddit.com/r/stm32f4/new.json",
        "https://www.reddit.com/r/sveltejs/new.json",
        "https://www.reddit.com/r/tutorials/new.json",
        "https://www.reddit.com/r/Unity3D/new.json",
        "https://www.reddit.com/r/unity_tutorials/new.json",
        "https://www.reddit.com/r/unrealengine/new.json",
        "https://www.reddit.com/r/warcraft3/new.json",
        "https://www.reddit.com/r/wildhearthstone/new.json",
        "https://www.reddit.com/r/wow/new.json",
    ];
    */
