use std::thread;
use std::time::Instant;

#[path = "./providers/fetch_and_map_to_classes/reddit_fetch_and_map.rs"]
mod reddit_fetch_and_map;
use reddit_fetch_and_map::reddit_part;
#[path = "./providers/fetch_and_map_to_classes/arxiv_fetch_and_map.rs"]
mod arxiv_fetch_and_map;
use arxiv_fetch_and_map::arxiv_part;
#[path = "./providers/fetch_and_map_to_classes/biorxiv_fetch_and_map.rs"]
mod biorxiv_fetch_and_map;
use biorxiv_fetch_and_map::biorxiv_part;
#[path = "./providers/fetch_and_map_to_classes/medrxiv_fetch_and_map.rs"]
mod medrxiv_fetch_and_map;
use medrxiv_fetch_and_map::medrxiv_part;

fn main() {
    let time = Instant::now();
    let mut threads_vec = vec![];
    threads_vec.push(thread::spawn(move || {
        reddit_part();
    }));
    threads_vec.push(thread::spawn(move || {
        arxiv_part();
        // let biorxiv_vec = arxiv_part();
        // for (key, value) in biorxiv_vec {
        //     if value.items.len() < 0 {
        //         print!("no value for key = {}\n", key,)
        //     }
        // }
    }));
    threads_vec.push(thread::spawn(move || {
        medrxiv_part(); //TODO паника тут!!!
    }));
    threads_vec.push(thread::spawn(move || {
        biorxiv_part();
        // let biorxiv_vec = biorxiv_part();
        // for (key, value) in biorxiv_vec {
        //     print!("{:#?}\n", key);
        // }
    }));
    for i in threads_vec {
        i.join().unwrap();
    }
    println!("main done in {} seconds", time.elapsed().as_secs());
}
