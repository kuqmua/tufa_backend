use std::collections::HashMap;
#[path = "../parsing/arxiv/parse_arxiv/get_arxiv_posts.rs"]
mod get_arxiv_posts;
use get_arxiv_posts::get_arxiv_posts;

#[path = "./get_group_names/get_arxiv_links_in_hash_map.rs"]
mod get_arxiv_links_in_hash_map;
use get_arxiv_links_in_hash_map::get_arxiv_links_in_hash_map;

#[path = "../parsing/arxiv/arxiv_xml_structs/arxiv_post.rs"]
mod arxiv_post;
use arxiv_post::ArxivPost;

use std::time::Instant;

pub fn reddit_part() -> HashMap<&'static str, Vec<ArxivPost>> {
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links_in_hash_map();
    let mut arxiv_pages_posts_hashmap: HashMap<&str, Vec<ArxivPost>> = HashMap::new();
    println!("{:#?}", arxiv_links_in_hash_map.len());
    let mut increment_for_errors = 0;
    for (key, value) in arxiv_links_in_hash_map {
        println!("starting to fetch data with index {}", increment_for_errors);
        let time = Instant::now();
        let result_of_arxiv_posts: Result<Vec<ArxivPost>, Box<dyn std::error::Error>> =
            get_arxiv_posts(value);
        match result_of_arxiv_posts {
            Ok(vec_of_arxiv_page_posts) => {
                arxiv_pages_posts_hashmap.insert(key, vec_of_arxiv_page_posts);
                println!(
                    "iteration with index {} just fetched ok data",
                    increment_for_errors
                );
                increment_for_errors += 1;
            }
            Err(e) => {
                println!(
                    "error = {}, iteration with index  = {}",
                    e, increment_for_errors
                );
                increment_for_errors += 1;
            }
        }
        println!(
            "iteration working/fetching(in seconds) = {} ",
            time.elapsed().as_secs()
        );
        //map.remove(key);
    }
    println!("{:#?}", arxiv_pages_posts_hashmap);
    arxiv_pages_posts_hashmap
}
