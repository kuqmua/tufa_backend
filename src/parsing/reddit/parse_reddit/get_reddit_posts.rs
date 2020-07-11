/*
use futures::{stream, StreamExt}; // 0.3.1
use reqwest::Client; // 0.10.0
use tokio; // 0.2.4, features = ["macros"]
*/
use std::thread;
use std::time::Duration;

extern crate roux;

use roux::Subreddit;

#[path = "../subreddit_rust_structs/reddit_post_data_wrapper.rs"]
mod reddit_post_data_wrapper;
use reddit_post_data_wrapper::RedditPostDataWrapper;

pub fn get_reddit_posts(subreddits_vec: Vec<&str>) -> Vec<RedditPostDataWrapper> {
    if subreddits_vec.len() >= 4294967295 {
        panic!("subreddits_vec.len() > 4294967295(u32::MAX)");
    }
    let handle = thread::spawn(|| {
        for i in 1..100 {
            println!("hi number {} first thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    let second_handle = thread::spawn(|| {
        for i in 1..100 {
            println!("hi number {} second thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let mut vec_reddit_post_data: Vec<RedditPostDataWrapper> =
        vec![RedditPostDataWrapper::new(); subreddits_vec.len()];
    let subreddit_names_vec: Vec<Subreddit> = push_names_into_subreddit_names_vec(&subreddits_vec);

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
    handle.join().unwrap();
    second_handle.join().unwrap();
    vec_reddit_post_data
}
/*
pub fn fetch_subreddit_posts(subreddit: Subreddit) {
    let latest = subreddit.latest(1, None);
    let unwrapped_latest = &latest.unwrap();
    &unwrapped_latest.data;
}
*/

pub fn parse_subreddit_post(subreddit: &Subreddit) -> RedditPostDataWrapper {
    //let data = fetch_subreddit_posts(subreddit);
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
