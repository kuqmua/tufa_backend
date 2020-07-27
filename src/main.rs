#[path = "providers/initialization/reddit_part.rs"]
mod reddit_part;
use reddit_part::reddit_part;
use std::thread;
/*
#[path = "providers/initialization/arxiv_init.rs"]
mod arxiv_init;
use arxiv_init::arxiv_init;
*/
use std::time::Instant;
#[path = "providers/providers_authorization/all_providers_authorization.rs"]
mod all_providers_authorization; //пока только РЕДДИТ ЧАСТЬ

#[path = "test_arxiv.rs"]
mod test_arxiv;
use test_arxiv::test_arxiv;
fn main() {
    let time = Instant::now();
    all_providers_authorization::all_providers_authorization();
    let reddit = thread::spawn(move || {
        reddit_part();
    });
    let arxiv = thread::spawn(move || {
        test_arxiv();
    });
    let reddit_res = reddit.join();
    let arxiv_res = arxiv.join();
    println!("main done in {} seconds", time.elapsed().as_secs());
}
