extern crate roux;
use roux::Subreddit;
use std::thread;
use std::time::Instant;
use tokio;

#[path = "parsing/reddit/reddit_json_structs/used_reddit_json_struct.rs"]
mod used_reddit_json_struct;
use used_reddit_json_struct::UsedRedditJsonStruct;

#[tokio::main]
pub async fn get_reddit_posts(subreddits_vec: Vec<&str>) -> Vec<UsedRedditJsonStruct> {
    if subreddits_vec.len() >= 4294967295 {
        panic!("subreddits_vec.len() > 4294967295(u32::MAX)");
    }
    let time = Instant::now();
    let subreddit_names_vec: Vec<Subreddit> = push_names_into_subreddit_names_vec(&subreddits_vec);
    let mut vec_reddit_post_data: Vec<UsedRedditJsonStruct> = vec![];
    let bodies = future::join_all(urls.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    }))
    .await;
    println!("{}", bodies.len());
    for b in bodies {
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let u: CastedRedditJsonStruct = serde_json::from_slice(slice).unwrap();
                println!(
                    "u.data.children[0].data.author {}",
                    u.data.children[0].data.author
                );
                println!(
                    "u.data.children[0].data.subreddit {}",
                    u.data.children[0].data.subreddit
                );
            }
            Err(e) => eprintln!("Got an error: {}", e),
        }
    }
    println!("time.elapsed().as_secs() = {}\n", time.elapsed().as_secs());
}

pub fn parse_subreddit_post(subreddit: &Subreddit) -> UsedRedditJsonStruct {
    let latest = subreddit.latest(1, None);
    let unwrapped_latest = &latest.unwrap();
    let data = &unwrapped_latest.data;
    let children = &data.children;
    let first_child = &children.first();

    let mut redditwrapper = UsedRedditJsonStruct::new();
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
let mut children = vec![];
    let mut count: usize = 0;
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
    */
