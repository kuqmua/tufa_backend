extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use futures::future;
use reqwest::Client;
use std::time::Instant;

use crate::config::ENABLE_ERROR_PRINTS_REDDIT;
use crate::config::ENABLE_PRINTS_REDDIT;
use crate::fetch::reddit_fetch_wrapper::parse_every_children::parse_every_children;
use crate::fetch::reddit_fetch_wrapper::push_names_into_two_layer_result_vec::push_names_into_two_layer_result_vec;
use crate::fetch::reddit_fetch_wrapper::reddit_json_structs::json_reddit_parser_struct::JsonRedditParserStruct;
use crate::fetch::reddit_fetch_wrapper::reddit_json_structs::json_reddit_parser_struct::JsonRedditParserStructVectorChild;
use crate::fetch::reddit_fetch_wrapper::reddit_json_structs::reddit_json_struct_vector::RedditJsonStructVector;
use crate::fetch::reddit_fetch_wrapper::subreddits_into_links::subreddits_into_links;
use crate::overriding::prints::print_error_red;

#[tokio::main]
pub async fn get_reddit_posts(subreddits: Vec<&str>) -> Vec<RedditJsonStructVector> {
    if subreddits.len() >= 4294967295 {
        panic!("subreddits_vec.len() > 4294967295(u32::MAX)");
    }
    let time = Instant::now();
    let client = Client::new();
    let mut two_layer_result_vec: Vec<RedditJsonStructVector> =
        push_names_into_two_layer_result_vec(&subreddits);
    let subreddits_links: Vec<String> = subreddits_into_links(subreddits);
    let bodies = future::join_all(subreddits_links.into_iter().map(|link| {
        let client = &client;
        async move {
            let resp = client.get(&link).send().await?;
            resp.bytes().await
        }
    }))
    .await;
    if ENABLE_PRINTS_REDDIT {
        //enable_time_measurement
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
                let u: JsonRedditParserStruct = serde_json::from_slice(slice).unwrap();
                if u.data.children.len() > 0 {
                    let children: &Vec<JsonRedditParserStructVectorChild> = &u.data.children;
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
