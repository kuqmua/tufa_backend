#[path = "providers_authorization/all_providers_authorization.rs"]
mod all_providers_authorization;

#[path = "parsing/reddit/parse_reddit/get_reddit_posts.rs"]
mod get_reddit_posts;

fn main() {
    all_providers_authorization::all_providers_authorization();
    get_reddit_posts::get_reddit_posts();
    //let article_id = &hot.unwrap().data.children.first().unwrap().data.id.clone();
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
