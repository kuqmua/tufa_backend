/*
#[path = "providers/initialization/reddit_part.rs"]
mod reddit_part;
use reddit_part::reddit_part;
*/
/*
#[path = "providers/initialization/arxiv_init.rs"]
mod arxiv_init;
use arxiv_init::arxiv_init;
*/
/*
#[path = "providers/providers_authorization/all_providers_authorization.rs"]
mod all_providers_authorization; //пока только РЕДДИТ ЧАСТЬ
*/
#[path = "test_arxiv.rs"]
mod test_arxiv;
use test_arxiv::test_arxiv;
fn main() {
    //all_providers_authorization::all_providers_authorization();
    //reddit_part();
    //arxiv_init();
    test_arxiv();
}
