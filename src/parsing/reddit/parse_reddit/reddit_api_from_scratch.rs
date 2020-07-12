extern crate reqwest;
extern crate serde_json;
use futures::{stream, StreamExt}; // 0.3.1
use reqwest::Client; // 0.10.0
use std::time::Instant;
use tokio; // 0.2.4, features = ["macros"]

#[path = "../subreddit_rust_structs/reddit_post_data_wrapper.rs"]
mod reddit_post_data_wrapper;
use reddit_post_data_wrapper::RedditPostDataWrapper;

//use crate::util::{FeedOption, RouxError};
//mod responses;
//use responses::{Comments, Moderators, Submissions};

fn from_subreddit_name_to_url(subreddit_name: &str) -> String {
    let subreddit_url = format!("https://www.reddit.com/r/{}", name, "new.json");
}
///////////

const PARALLEL_REQUESTS: usize = 8;

#[tokio::main]
async fn something() {
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
    let client = Client::new();
    let before = Instant::now();
    let subreddit_urls = vec![
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

    let bodies = stream::iter(subreddit_urls)
        .map(|subreddit_url| {
            let client = &client;
            async move {
                let resp = client.get(subreddit_url).send().await?;
                resp.bytes().await
            }
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => println!("Got {:#?} ", b),
                Err(e) => eprintln!("Got an error: {}", e),
            }
        })
        .await;
    println!("{}", before.elapsed().as_secs());
}
/////////////////////
/// Subreddit.
pub struct Subreddit {
    /// Name of subreddit.
    pub name: String,
    url: String,
    client: Client,
}

impl Subreddit {
    /// Create a new `Subreddit` instance.
    pub fn new(name: &str) -> Subreddit {
        let subreddit_url = format!("https://www.reddit.com/r/{}", name);

        Subreddit {
            name: name.to_owned(),
            url: subreddit_url,
            client: Client::new(),
        }
    }
    fn get_feed(
        &self,
        ty: &str,
        limit: u32,
        options: Option<FeedOption>,
    ) -> Result<Submissions, RouxError> {
        let url = &mut format!("{}/{}.json?limit={}", self.url, ty, limit);

        if !options.is_none() {
            let option = options.unwrap();

            if !option.after.is_none() {
                url.push_str(&mut format!("&after={}", option.after.unwrap().to_owned()));
            } else if !option.before.is_none() {
                url.push_str(&mut format!(
                    "&before={}",
                    option.before.unwrap().to_owned()
                ));
            }

            if !option.count.is_none() {
                url.push_str(&mut format!("&count={}", option.count.unwrap()));
            }
        }

        Ok(self
            .client
            .get(&url.to_owned())
            .send()?
            .json::<Submissions>()?)
    }

    /// Get latest posts.
    pub fn latest(
        &self,
        limit: u32,
        options: Option<FeedOption>,
    ) -> Result<Submissions, RouxError> {
        self.get_feed("new", limit, options)
    }

   