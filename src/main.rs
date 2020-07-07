extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
mod from_rust_to_json_structs;

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
/*

fn get_feed(url: &str) -> Origin {
    let client = reqwest::Client::new();
    let mut request = client.get(url);

    let mut resp = request.send().unwrap();

    assert!(resp.status().is_success());

    let json = resp.text().unwrap();
    let v: Value = serde_json::from_str(&json).unwrap();

    println!("Please call {} ", v);
    serde_json::from_str(&json).unwrap()
}
*/

/*
#[derive(Debug, Deserialize, Serialize)]
struct Author {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FeedItem {
    id: String,
    title: String,
    content_text: String,
    url: String,
    date_published: String,
    author: Author,
}

#[derive(Debug, Deserialize, Serialize)]
struct Feed {
    version: String,
    title: String,
    home_page_url: String,
    feed_url: String,
    description: String,
    author: Author,
    items: Vec<FeedItem>,
}
*/
