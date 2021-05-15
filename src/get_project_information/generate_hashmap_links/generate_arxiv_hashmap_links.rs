pub fn generate_arxiv_hashmap_links(arxiv_names: Vec<&'static str>) -> Vec<String> {
    //example http://export.arxiv.org/rss/astro-ph.CO
    let first_part_of_link: &str = "http://export.arxiv.org/rss/";
    let mut arxiv_links: Vec<String> = Vec::with_capacity(arxiv_names.len());
    for value in arxiv_names {
        arxiv_links.push(format!("{}{}", first_part_of_link, value));
    }
    println!("arxiv_links{:#?}", arxiv_links);
    arxiv_links
}
