#[path = "providers/initialization/reddit_part.rs"]
mod reddit_part;
use reddit_part::reddit_part;

fn main() {
    reddit_part();
}
