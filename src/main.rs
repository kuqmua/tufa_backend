/*
#[path = "providers_authorization/all_providers_authorization.rs"]
mod all_providers_authorization;

#[path = "parsing/reddit/parse_reddit/get_reddit_posts.rs"]
mod get_reddit_posts;
/*
#[path = "parsing/reddit/parse_reddit/reddit_api_from_scratch.rs"]
mod reddit_api_from_scratch;
*/
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
    let vec_reddit_posts = get_reddit_posts::get_reddit_posts(subreddits_names);
    println!("{}", vec_reddit_posts[0])
}
*/

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;

//use reqwest::
use futures::future; // 0.3.4
use reqwest::Client; // 0.10.1
use tokio; // 0.2.11
           /*
           #[path = "parsing/reddit/subreddit_rust_structs/reddit_post_data_wrapper.rs"]
           mod reddit_post_data_wrapper;
           //use reddit_post_data_wrapper::RedditPostDataWrapper;
           use reddit_post_data_wrapper::Root;
           */
//use roux::Subreddit;
use std::time::Instant;

#[path = "parsing/reddit/subreddit_rust_structs/from_reddit_to_json_structs.rs"]
mod from_reddit_to_json_structs;
use from_reddit_to_json_structs::Root;
/*
#[derive(Deserialize)]
struct Ip {
    origin: String,
}
*/
#[tokio::main]
async fn main() {
    //let jawait = reqwest::get("http://httpbin.org/ip").await?;
    //let json: Ip = jawait.json?;
    //println!("{:?}", json);
    let time = Instant::now();

    let client = Client::new();

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
    /*
    let urls = vec![
        "https://jsonplaceholder.typicode.com/todos/1",
    ];
    */
    let bodies = future::join_all(urls.into_iter().map(|url| {
        let client = &client;
        async move {
            //let subreddit = Subreddit::new("wow");
            //let hot = subreddit.hot(25, None);
            //let article_id = &hot.unwrap().data.children.first().unwrap().data.id.clone();
            let resp = client.get(url).send().await?;
            //let resp = client.get(url)?.json()?;
            resp.bytes().await
        }
    }))
    .await;
    println!("{}", bodies.len());
    for b in bodies {
        match b {
            Ok(b) => {
                //println!("Got {} bytes", b.len());
                let slice: &[u8] = &b;
                //let string_u = &b.as_string();
                //тут нужно в стрингу загнать
                let u: Root = serde_json::from_slice(slice).unwrap();
                println!(
                    "u.data.children[0].data.author {}",
                    u.data.children[0].data.author
                );
                println!(
                    "u.data.children[0].data.subreddit {}",
                    u.data.children[0].data.subreddit
                );
                //println!("u.kind {:#?}", u);
                //println!("u.kind {}", u.kind);
                //println!("{:?} ", u);
                /*
                let u: Option = serde_json::from_slice(slice).unwrap();
                match u {
                    Ok(b) => {
                        println!("ok");
                    }
                    Err(e) => eprintln!("error:"),
                }
                println!("{:#?}", u);
                //wtf = b.json::<RedditPostDataWrapper>();
                */
            }
            Err(e) => eprintln!("Got an error: {}", e),
        }
    }
    println!("time.elapsed().as_secs() = {}\n", time.elapsed().as_secs());
}
