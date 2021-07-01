pub fn generate_arxiv_hashmap_links(arxiv_names: Vec<String>) -> Vec<String> {
    //example http://export.arxiv.org/rss/astro-ph.CO
    println!("arxiv_names 1************* {:#?}", arxiv_names);
    let mut f: Vec<String> = Vec::with_capacity(arxiv_names.len());
    for i in &arxiv_names {
        f.push(i.replace("\"", ""));
    }
    println!("f 2************* {:#?}", f);
    let first_part_of_link: &str = "http://export.arxiv.org/rss/";
    let mut arxiv_links: Vec<String> = Vec::with_capacity(arxiv_names.len());
    for value in f {
        arxiv_links.push(format!("{}{}", first_part_of_link, value));
    }
    arxiv_links
}
