extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use futures::future;
use reqwest::Client;
use serde_json::json;
use std::str;
use std::time::Instant;
use tokio;

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct Root {
//     pub posts: Vec<Post>,
// }

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub creator: String,
    pub title: String,
    pub link: String,
    pub pub_date: String,
    #[serde(rename = "content:encoded")]
    pub content_encoded: String,
    #[serde(rename = "dc:creator")]
    pub dc_creator: String,
    pub guid: String,
    pub categories: Vec<String>,
    pub iso_date: String,
}

#[tokio::main]
pub async fn medium() -> u32 {
    let time = Instant::now();
    let client = Client::new();

    let subreddits_urls: Vec<String> =
        ["https://v1.nocodeapi.com/tufa/medium/PlRDBfkCKCpvgKGn".to_string(); 1].to_vec();
    let bodies = future::join_all(subreddits_urls.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(&url).send().await?;
            resp.bytes().await
        }
    }))
    .await;
    println!(
        "reddit future::join_all (in seconds) = {} ",
        time.elapsed().as_secs()
    );
    println!("get_reddit_posts bodies.len() {}", bodies.len());
    // let mut count = 0;
    for b in bodies {
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let u: Vec<Root> = serde_json::from_slice(slice).unwrap();
                let v = str::from_utf8(&slice).unwrap();
                //let v: Root = serde_json::from_str(u).unwrap();
                print!("{:#?}~~~", u[0].title);
                // if u.posts.len() > 0 {
                //     print!("{:#?}", u);
                // } else {
                //     print!("u.data.children.len() > 0 NOPE");
                // }
                // count += 1;
            }
            Err(e) => {
                // count += 1;
                eprintln!("Got an error: {}", e)
            }
        }
    }
    let s = 5;
    s
}
