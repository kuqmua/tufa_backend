//#![feature(proc_macro_hygiene, stmt_expr_attributes)]
//use futures::{stream, StreamExt}; // 0.3.1
//use reqwest::Client; // 0.10.0
use std::thread;
use std::time::Instant;
use tokio; // 0.2.4, features = ["macros"]
           //use std::time::Duration;
           //use futures::stream::Stream;
           //use futures_async_stream::for_await;
extern crate roux;

use roux::Subreddit;

#[path = "../subreddit_rust_structs/reddit_post_data_wrapper.rs"]
mod reddit_post_data_wrapper;
use reddit_post_data_wrapper::RedditPostDataWrapper;

#[tokio::main]
pub async fn get_reddit_posts(subreddits_vec: Vec<&str>) -> Vec<RedditPostDataWrapper> {
    if subreddits_vec.len() >= 4294967295 {
        panic!("subreddits_vec.len() > 4294967295(u32::MAX)");
    }
    //let client = Client::new();
    let time = Instant::now();
    let subreddit_names_vec: Vec<Subreddit> = push_names_into_subreddit_names_vec(&subreddits_vec);
    /*
    let mut vec_reddit_post_data: Vec<RedditPostDataWrapper> =
        vec![RedditPostDataWrapper::new(); subreddits_vec.len()];
    //let mut vec = Vec::new();
    let mut count: usize = 0;
    loop {
        if count < subreddits_vec.len() {
            let post = parse_subreddit_post(&subreddit_names_vec[count]);
            vec_reddit_post_data[count] = post;
            count += 1;
        } else {
            break;
        }
    }
    */
    /*
    let mut children = vec![];
    let number_of_threads: usize = 65;
    let mut count: usize = 0;
    for i in 0..number_of_threads {

        children.push(thread::spawn( move|| {
            let post = parse_subreddit_post(subreddit_names_vec[count]);
            vec_reddit_post_data[count] = post.clone();
            count += 1;

        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    */

    let mut vec_reddit_post_data: Vec<RedditPostDataWrapper> = vec![];
    let mut children = vec![];
    //let number_of_threads: usize = subreddits_vec.len();
    let mut count: usize = 0;
    //Arc<Mutex<>>
    loop {
        if count < subreddits_vec.len() {
            children.push(thread::spawn(move || {
                count += 1;
                let a = parse_subreddit_post(&subreddit_names_vec[count]);
                vec_reddit_post_data.push(a);
            }));
        } else {
            break;
        }
    }
    for child in children {
        let _ = child.join();
    }
    println!("{}", time.elapsed().as_secs());
    vec_reddit_post_data
}

/*
 fn stream_subreddit_post(stream: impl Stream<Item = i32>)-> Vec<RedditPostDataWrapper>{
     let mut vec = Vec::new();
    #[for_await]
    for value in stream {
        vec.push(value);
    }
    vec
 }
*/

pub fn parse_subreddit_post(subreddit: &Subreddit) -> RedditPostDataWrapper {
    let latest = subreddit.latest(1, None);
    let unwrapped_latest = &latest.unwrap();
    let data = &unwrapped_latest.data;
    let children = &data.children;
    let first_child = &children.first();

    let mut redditwrapper = RedditPostDataWrapper::new();
    match first_child {
        Some(_) => {
            let unwrapped_first_child = first_child.unwrap();
            let first_child_data = &unwrapped_first_child.data;
            redditwrapper.subreddit = first_child_data.subreddit.clone();
            redditwrapper.selftext = first_child_data.selftext.clone();
            redditwrapper.id = first_child_data.id.clone();
            redditwrapper.author = first_child_data.author.clone();
            redditwrapper.title = first_child_data.title.clone();
            redditwrapper.domain = first_child_data.domain.clone();
            redditwrapper.permalink = first_child_data.permalink.clone();
            match &first_child_data.url {
                Some(x) => redditwrapper.url = Some(x.clone()),
                None => redditwrapper.url = Some("None".to_string()),
            }
            redditwrapper.thumbnail = first_child_data.thumbnail.clone();
            redditwrapper.created_utc = first_child_data.created_utc.clone();
            redditwrapper.ups = first_child_data.ups.clone();
            redditwrapper.num_comments = first_child_data.num_comments.clone();
            redditwrapper.over_18 = first_child_data.over_18.clone();
            redditwrapper.score = first_child_data.score.clone();
            redditwrapper.quarantine = first_child_data.quarantine.clone();
            redditwrapper.is_self = first_child_data.is_self.clone();
            redditwrapper.saved = first_child_data.saved.clone();
        }
        None => println!("No first child"),
    }
    println!("redditwrapper ------------ {}", redditwrapper.subreddit);
    //println!("redditwrapper ------------ {:#?}", redditwrapper);
    redditwrapper
}

pub fn push_names_into_subreddit_names_vec(subreddits_vec: &Vec<&str>) -> Vec<Subreddit> {
    let mut subreddit_names_vec: Vec<Subreddit> = Vec::with_capacity(subreddits_vec.len());
    for subreddititer in subreddits_vec {
        subreddit_names_vec.push(Subreddit::new(&subreddititer));
    }
    subreddit_names_vec
}

/*
    let handle = thread::spawn(|| {
        for i in 1..100 {
            println!("hi number {} first thread!", i);
        }
    });
    let second_handle = thread::spawn(|| {
        for i in 1..100 {
            println!("hi number {} second thread!", i);
        }
    });
*/
/*
use futures::{stream, StreamExt}; // 0.3.1
use reqwest::Client; // 0.10.0
use std::time::Instant;
use tokio; // 0.2.4, features = ["macros"]

const PARALLEL_REQUESTS: usize = 8;

fn main() {
    something();
}
#[tokio::main]
async fn something() {
    let client = Client::new();
    let before = Instant::now();
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
subreddits_vec
    let bodies = stream::iter(subreddits_vec)
        .map(|subreddits_vec_member| {
            let client = &client;
            async move {
                let post = parse_subreddit_post(&subreddit_names_vec[count]);
            vec_reddit_post_data[count] = post;
                let resp = client.get(url).send().await?;
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
*/
/*
const PARALLEL_REQUESTS: usize = 8;
let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = client.get(url).send().await?;
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
*/
