extern crate roux;

use roux::Subreddit;

pub fn get_reddit_posts() {
    let subreddit = Subreddit::new("wow");
    let hot = subreddit.hot(15, None);
    let data = &hot.unwrap().data.children;
    let mut count: u8 = 0;
    let mut authors = Vec::new();

    for child in data {
        count += 1;
        let author: String = child.data.author.clone();
        let title: String = child.data.title.clone();
        let ups: f64 = child.data.ups.clone();
        println!("author = {}, selftext = {}, ups = {}", author, title, ups);
        authors.push(author);
    }
    println!("count = {}", count);
    println!("authors.len() = {}", authors.len());
}
