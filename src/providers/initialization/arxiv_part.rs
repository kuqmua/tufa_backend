#[path = "../parsing/arxiv/parse_arxiv/get_arxiv_posts.rs"]
mod get_arxiv_posts;
use get_arxiv_posts::get_arxiv_posts;

#[path = "./get_group_names/get_arxiv_links_in_hash_map.rs"]
mod get_arxiv_links_in_hash_map;
use get_arxiv_links_in_hash_map::get_arxiv_links_in_hash_map;

pub fn reddit_part() {
    let arxiv_links_in_hash_map: HashMap<&str, &str> = get_arxiv_links_in_hash_map();
    let vec = get_arxiv_posts(arxiv_links_in_hash_map);
    println!("some");
}
