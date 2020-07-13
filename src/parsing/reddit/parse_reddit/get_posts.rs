extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use futures::future;
use reqwest::Client;
use std::time::Instant;
use tokio;

#[path = "../reddit-json-structs/casted.rs"]
mod casted;
use casted::CastedRedditJsonStruct;

#[path = "../reddit-json-structs/used.rs"]
mod used;
use used::UsedRedditJsonStruct;

//
#[tokio::main]
pub async fn get_posts(subreddits: Vec<&str>) -> Vec<UsedRedditJsonStruct> {
    if subreddits.len() >= 4294967295 {
        panic!("subreddits_vec.len() > 4294967295(u32::MAX)");
    }
    let time = Instant::now();
    let client = Client::new();
    let mut result_vec: Vec<UsedRedditJsonStruct> =
        push_names_into_subreddit_names_vec(&subreddits);
    let subreddits_urls: Vec<String> = subreddits_into_urls(subreddits);
    let bodies = future::join_all(subreddits_urls.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(&url).send().await?;
            resp.bytes().await
        }
    }))
    .await;
    println!(" bodies.len() {}", bodies.len());

    let mut count = 0;
    for b in bodies {
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let u: CastedRedditJsonStruct = serde_json::from_slice(slice).unwrap();
                println!("u.data.children.len() {}", u.data.children.len());
                result_vec[count].author = u.data.children[0].data.author.clone();
                count += 1;
            }
            Err(e) => {
                count += 1;
                eprintln!("Got an error: {}", e)
            }
        }
    }
    println!("time.elapsed().as_secs() = {}", time.elapsed().as_secs());
    result_vec
}

fn subreddits_into_urls(subreddits: Vec<&str>) -> Vec<String> {
    let start: &str = "https://www.reddit.com/r/";
    let end: &str = "/new.json";
    let mut subreddits_urls = Vec::with_capacity(subreddits.len());
    for subreddit in subreddits {
        let subreddit_url = format!("{}{}{}", start, subreddit, end);
        subreddits_urls.push(subreddit_url)
    }
    subreddits_urls
}

pub fn push_names_into_subreddit_names_vec(
    subreddits_vec: &Vec<&str>,
) -> Vec<UsedRedditJsonStruct> {
    let mut subreddit_names_vec: Vec<UsedRedditJsonStruct> =
        Vec::with_capacity(subreddits_vec.len());
    let mut count = 0;
    while count < subreddits_vec.len() {
        subreddit_names_vec.push(UsedRedditJsonStruct::new());
        count += 1;
    }
    subreddit_names_vec
}
