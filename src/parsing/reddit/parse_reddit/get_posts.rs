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
use casted::Children;

#[path = "../reddit-json-structs/used.rs"]
mod used;
use used::UsedRedditJsonStruct;
use used::VecOfUsedRedditJsonStruct;
//
#[tokio::main]
pub async fn get_posts(subreddits: Vec<&str>) -> Vec<VecOfUsedRedditJsonStruct> {
    if subreddits.len() >= 4294967295 {
        panic!("subreddits_vec.len() > 4294967295(u32::MAX)");
    }
    let time = Instant::now();
    let client = Client::new();
    let mut two_layer_result_vec: Vec<VecOfUsedRedditJsonStruct> =
        push_names_into_two_layer_result_vec(&subreddits);
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
                if u.data.children.len() > 0 {
                    //let subreddit = u.data.children[0].data.subreddit; //неважно какой child тут, сабреддит одиаковый же
                    let children: &Vec<Children> = &u.data.children;
                    two_layer_result_vec[count] = parse_every_children(&u, &children);
                    println!("{}", two_layer_result_vec[count].posts[0].author);
                } else {
                    print!("u.data.children.len() > 0 NOPE");
                }
                count += 1;
            }
            Err(e) => {
                count += 1;
                eprintln!("Got an error: {}", e)
            }
        }
    }
    println!("time.elapsed().as_secs() = {}", time.elapsed().as_secs());
    return two_layer_result_vec;
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
/*
fn push_names_into_subreddit_names_vec(subreddits_vec: &Vec<&str>) -> Vec<UsedRedditJsonStruct> {
    let mut subreddit_names_vec: Vec<UsedRedditJsonStruct> =
        Vec::with_capacity(subreddits_vec.len());
    let mut count = 0;
    while count < subreddits_vec.len() {
        subreddit_names_vec.push(UsedRedditJsonStruct::new());
        count += 1;
    }
    subreddit_names_vec
}
*/
fn push_names_into_two_layer_result_vec(
    subreddits_vec: &Vec<&str>,
) -> Vec<VecOfUsedRedditJsonStruct> {
    let mut subreddit_names_vec: Vec<VecOfUsedRedditJsonStruct> =
        Vec::with_capacity(subreddits_vec.len());
    let mut count = 0;
    while count < subreddits_vec.len() {
        subreddit_names_vec.push(VecOfUsedRedditJsonStruct::new());
        count += 1;
    }
    subreddit_names_vec
}

// result_vec: Vec<VecOfUsedRedditJsonStruct>,
//count: usize,
fn parse_every_children(
    u: &CastedRedditJsonStruct,
    children: &Vec<Children>,
) -> VecOfUsedRedditJsonStruct {
    let mut vec_of_children = VecOfUsedRedditJsonStruct::new();
    let mut count = 0;
    while count < children.len() {
        let mut child = UsedRedditJsonStruct::new();
        child.url = u.data.children[count].data.url.clone();
        child.subreddit = u.data.children[count].data.subreddit.clone();
        child.id = u.data.children[count].data.id.clone();
        child.author = u.data.children[count].data.author.clone();
        child.title = u.data.children[count].data.title.clone();
        child.domain = u.data.children[count].data.domain.clone();
        child.permalink = u.data.children[count].data.permalink.clone();
        child.thumbnail = u.data.children[count].data.thumbnail.clone();
        child.created_utc = u.data.children[count].data.created_utc.clone();
        child.ups = u.data.children[count].data.ups.clone();
        child.score = u.data.children[count].data.score.clone();
        child.num_comments = u.data.children[count].data.num_comments.clone();
        child.over_18 = u.data.children[count].data.over_18.clone();
        child.quarantine = u.data.children[count].data.quarantine.clone();
        child.is_self = u.data.children[count].data.is_self.clone();
        child.saved = u.data.children[count].data.saved.clone();
        vec_of_children.posts.push(child);
        count += 1;
    }
    vec_of_children
}
/*
result_vec[count].url = u.data.children[0].data.url.clone();
                result_vec[count].subreddit = u.data.children[0].data.subreddit.clone();
                result_vec[count].id = u.data.children[0].data.id.clone();
                result_vec[count].author = u.data.children[0].data.author.clone();
                result_vec[count].title = u.data.children[0].data.title.clone();
                result_vec[count].domain = u.data.children[0].data.domain.clone();
                result_vec[count].permalink = u.data.children[0].data.permalink.clone();
                result_vec[count].thumbnail = u.data.children[0].data.thumbnail.clone();
                result_vec[count].created_utc = u.data.children[0].data.created_utc.clone();
                result_vec[count].ups = u.data.children[0].data.ups.clone();
                result_vec[count].score = u.data.children[0].data.score.clone();
                result_vec[count].num_comments = u.data.children[0].data.num_comments.clone();
                result_vec[count].over_18 = u.data.children[0].data.over_18.clone();
                result_vec[count].quarantine = u.data.children[0].data.quarantine.clone();
                result_vec[count].is_self = u.data.children[0].data.is_self.clone();
                result_vec[count].saved = u.data.children[0].data.saved.clone();
*/
