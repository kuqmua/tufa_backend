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
    println!("{}", vec[1].posts[2].author,);
}
/*
vec[0].posts[0].url,
        vec[0].posts[0].subreddit,
        vec[0].posts[0].selftext,
        vec[0].posts[0].id,
vec[0].posts[0].title,
        vec[0].posts[0].domain,
        vec[0].posts[0].permalink,
        vec[0].posts[0].thumbnail,
        vec[0].posts[0].created_utc,
        vec[0].posts[0].ups,
        vec[0].posts[0].score,
        vec[0].posts[0].num_comments,
        vec[0].posts[0].over_18,
        vec[0].posts[0].quarantine,
        vec[0].posts[0].is_self,
        vec[0].posts[0].saved,
*/
