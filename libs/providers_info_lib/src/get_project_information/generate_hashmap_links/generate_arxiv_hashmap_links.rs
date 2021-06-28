// extern crate mongo_demo;

// use futures::executor::block_on;
pub fn generate_arxiv_hashmap_links(arxiv_names: Vec<&'static str>) -> Vec<String> {
    // #[tokio::main]
    // async fn main() {

    let result_of_mongo_integration = mongo_integration::mongo_integration();
    match result_of_mongo_integration {
        Ok(()) => {
            println!("nice!");
        }
        Err(e) => {
            println!("F");
        }
    }

    // }
    //example http://export.arxiv.org/rss/astro-ph.CO
    let first_part_of_link: &str = "http://export.arxiv.org/rss/";
    let mut arxiv_links: Vec<String> = Vec::with_capacity(arxiv_names.len());
    for value in arxiv_names {
        arxiv_links.push(format!("{}{}", first_part_of_link, value));
    }
    arxiv_links
}
