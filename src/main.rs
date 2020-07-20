#[path = "providers/initialization/reddit_part.rs"]
mod reddit_part;
use reddit_part::reddit_part;

#[path = "providers/initialization/arxiv_part.rs"]
mod arxiv_part;
use arxiv_part::arxiv_part;

#[path = "providers/providers_authorization/all_providers_authorization.rs"]
mod all_providers_authorization; //пока только РЕДДИТ ЧАСТЬ

fn main() {
    all_providers_authorization::all_providers_authorization();
    reddit_part();
    arxiv_part();
}
