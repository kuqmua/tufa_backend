extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use futures::future;
use reqwest::Client;
use std::time::Instant;
use tokio;

use crate::fetch::reddit_fetch::parse_every_children::parse_every_children;
use crate::fetch::reddit_fetch::push_names_into_two_layer_result_vec::push_names_into_two_layer_result_vec;
use crate::fetch::reddit_fetch::reddit_json_structs::casted::CastedRedditJsonStruct;
use crate::fetch::reddit_fetch::reddit_json_structs::casted::Children;
use crate::fetch::reddit_fetch::reddit_json_structs::used::VecOfUsedRedditJsonStruct;
use crate::fetch::reddit_fetch::subreddits_into_urls::subreddits_into_urls;
use crate::config::ENABLE_ERROR_PRINTS_REDDIT;
use crate::config::ENABLE_PRINTS_REDDIT;
use crate::override_prints::override_prints::print_error_red;

#[tokio::main]
pub async fn get_reddit_posts(subreddits: Vec<&str>) -> Vec<VecOfUsedRedditJsonStruct> {
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
    if ENABLE_PRINTS_REDDIT {
        println!(
            "fetch in {} seconds, reddit bodies: {} ",
            time.elapsed().as_secs(),
            bodies.len()
        );
    };
    let mut count = 0;
    for b in bodies {
        match b {
            Ok(b) => {
                let slice: &[u8] = &b;
                let u: CastedRedditJsonStruct = serde_json::from_slice(slice).unwrap();
                if u.data.children.len() > 0 {
                    let children: &Vec<Children> = &u.data.children;
                    two_layer_result_vec[count] = parse_every_children(&u, &children);
                //println!("{}", two_layer_result_vec[count].posts[0].author);
                } else {
                    if ENABLE_ERROR_PRINTS_REDDIT {
                        print_error_red(
                            file!().to_string(),
                            line!().to_string(),
                            "u.data.children.len() > 0 NOPE".to_string(),
                        )
                    }
                }
                count += 1;
            }
            Err(e) => {
                count += 1;
                if ENABLE_ERROR_PRINTS_REDDIT {
                    print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                }
            }
        }
    }
    if ENABLE_PRINTS_REDDIT {
        println!(
            "parsing in {} seconds, reddit bodies: {} ",
            time.elapsed().as_secs(),
            two_layer_result_vec.len()
        );
    };
    return two_layer_result_vec;
}
