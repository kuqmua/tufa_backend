extern crate roux;

use roux::Subreddit;

pub fn get_reddit_posts() {
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
    let mut count_subreddits: u8 = 0;
    for subreddititer in user_subreddits {
        count_subreddits += 1;
        let subreddit = Subreddit::new(subreddititer);
        println!("subreddit ------------------ {}", subreddititer);
        let hot = subreddit.hot(1, None);
        let data = &hot.unwrap().data.children;
        let mut count: u8 = 0;
        let mut authors = Vec::new();

        for child in data {
            count += 1;
            let author: String = child.data.author.clone();
            let title: String = child.data.title.clone();
            let ups: f64 = child.data.ups.clone();
            println!("author = {}, selftext = {}, ups = {}", author, title, ups);
            authors.push(author);
        }
        println!("count = {}", count);
    }
    println!("countSubreddits = {}", count_subreddits);
}
