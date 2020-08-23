use std::thread;
use std::time::Instant;
// #[path = "providers/initialization/reddit_part.rs"]
// mod reddit_part;
// use reddit_part::reddit_part;

// #[path = "providers/providers_authorization/all_providers_authorization.rs"]
// mod all_providers_authorization; //пока только РЕДДИТ ЧАСТЬ

// #[path = "arxiv_copy.rs"]
// mod arxiv_copy;
// use arxiv_copy::arxiv_part;

// #[path = "biorxiv_test.rs"]
// mod biorxiv_test;
// use biorxiv_test::biorxiv_part;

// #[path = "test_medrxiv.rs"]
// mod test_medrxiv;
// use test_medrxiv::medrxiv_part;

#[path = "medium.rs"]
mod medium;
use medium::medium;

fn main() {
    let time = Instant::now();
    let mut threads_vec = vec![];
    // all_providers_authorization::all_providers_authorization();
    // threads_vec.push(thread::spawn(move || {
    //     reddit_part();
    // }));
    // threads_vec.push(thread::spawn(move || {
    //     arxiv_part();
    // }));
    // threads_vec.push(thread::spawn(move || {
    //     medrxiv_part();
    // }));
    // threads_vec.push(thread::spawn(move || {
    //     biorxiv_part();
    // }));
    threads_vec.push(thread::spawn(move || {
        let f = medium();
        let d = f + 1;
        println!("d {}", d);
    }));
    for i in threads_vec {
        i.join().unwrap();
    }

    println!("main done in {} seconds", time.elapsed().as_secs());
}
