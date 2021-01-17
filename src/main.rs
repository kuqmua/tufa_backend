use std::thread;
use std::time::Instant;
// #[path = "providers/initialization/arxiv_init.rs"]//СТАРАЯ ВЕРСИЯ АРХИВА
// mod arxiv_init;
// use arxiv_init::arxiv_init;
// #[path = "providers/initialization/reddit_part.rs"]
// mod reddit_part;
// use reddit_part::reddit_part;
// #[path = "test_arxiv.rs"]//СТАРАЯ ВЕРСИЯ АРХИВА
// mod test_arxiv;
// use test_arxiv::test_arxiv;
#[path = "arxiv_copy.rs"]
mod arxiv_copy;
use arxiv_copy::arxiv_part;
// #[path = "biorxiv.rs"]
// mod biorxiv;
// use biorxiv::test_biorxiv;
// #[path = "biorxiv_test.rs"]
// mod biorxiv_test;
// use biorxiv_test::biorxiv_part;
// #[path = "test_medrxiv.rs"]
// mod test_medrxiv;
// use test_medrxiv::medrxiv_part;

fn main() {
    let time = Instant::now();
    let mut threads_vec = vec![];
    // threads_vec.push(thread::spawn(move || {
    //     reddit_part();
    // }));
    threads_vec.push(thread::spawn(move || {
        arxiv_part();
        // let biorxiv_vec = arxiv_part();
        // for (key, value) in biorxiv_vec {
        //     if value.items.len() < 0 {
        //         print!("no value for key = {}\n", key,)
        //     }
        // }
    }));
    // threads_vec.push(thread::spawn(move || {
    //     medrxiv_part(); //TODO паника тут!!!
    // }));
    // threads_vec.push(thread::spawn(move || {
    //     biorxiv_part();
    //     // let biorxiv_vec = biorxiv_part();
    //     // for (key, value) in biorxiv_vec {
    //     //     print!("{:#?}\n", key);
    //     // }
    // }));
    for i in threads_vec {
        i.join().unwrap();
    }
    println!("main done in {} seconds", time.elapsed().as_secs());
}
