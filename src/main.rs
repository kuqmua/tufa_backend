use std::thread;

/*
#[path = "providers/initialization/arxiv_init.rs"]
mod arxiv_init;
use arxiv_init::arxiv_init;
*/

// #[path = "providers/initialization/reddit_part.rs"]
// mod reddit_part;
// use reddit_part::reddit_part;

use std::time::Instant;
#[path = "providers/providers_authorization/all_providers_authorization.rs"]
mod all_providers_authorization; //пока только РЕДДИТ ЧАСТЬ

// #[path = "test_arxiv.rs"]
// mod test_arxiv;
// use test_arxiv::test_arxiv;

// #[path = "biorxiv.rs"]
// mod biorxiv;
// use biorxiv::test_biorxiv;

#[path = "biorxiv_test.rs"]
mod biorxiv_test;
use biorxiv_test::biorxiv_part;

fn main() {
    let time = Instant::now();
    all_providers_authorization::all_providers_authorization();
    let mut threads_vec = vec![];
    // threads_vec.push(thread::spawn(move || {
    //     reddit_part();
    // }));
    // threads_vec.push(thread::spawn(move || {
    //     test_arxiv();
    // }));
    // threads_vec.push(thread::spawn(move || {
    //     test_biorxiv();
    // }));
    threads_vec.push(thread::spawn(move || {
        let biorxiv_vec = biorxiv_part();
        for (key, value) in biorxiv_vec {
            print!("{:#?}\n", key);
        }
    }));
    for i in threads_vec {
        i.join().unwrap();
    }

    println!("main done in {} seconds", time.elapsed().as_secs());
}
