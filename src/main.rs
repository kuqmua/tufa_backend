extern crate reqwest;
extern crate roux;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
#[path = "parsing/subreddit_rust_structs/from_rust_to_json_structs.rs"]
mod from_rust_to_json_structs;
use roux::{Reddit, Subreddit};

pub static URL: &str = "https://www.reddit.com/r/books/hot.json";

fn main() {
    let client = reqwest::Client::new();
    let mut request = client.get(URL);

    let mut resp = request.send().unwrap();

    assert!(resp.status().is_success());

    let json = resp.text().unwrap();
    let v: from_rust_to_json_structs::Root = serde_json::from_str(&json).unwrap();
    println!(
        "v.data.children[0].kind = {}, v.data.children.len() = {}",
        v.data.children[0].kind,
        v.data.children.len()
    );
    let client = Reddit::new(
        "windows:roux:v0.3.0 (by /u/joigikuna)",
        "dE--I-gcvddUhA",
        "yphoPIwVa-Zqi8bO89Ks8dqdyEc",
    )
    .username("joigikuna")
    .password("loypure618sae")
    .login()
    .unwrap(); // вынести в функцию

    //let me = client.unwrap();
    let subreddit = Subreddit::new("wow");
    //let moderators = subreddit.moderators();
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
    //let article_id = &hot.unwrap().data.children.first().unwrap().data.id.clone();
    /*
    match hot {
        Submissions => Submissions,
        RouxError => print!("RouxError"),
    }
    */
    //print!("{}", std::mem::size_of_val(&hot));
    //print!("article_id ={}", article_id);
    //print!("article_id ={}", data);
}

/*
#[derive(Debug, Deserialize, Serialize)]
struct Origin {
    origin: String,
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.origin)
    }
}
*/
