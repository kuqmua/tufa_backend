use std::thread;

use crate::config::ENABLE_ARXIV;
use crate::fetch;
// use config::ENABLE_BIORXIV;
// use config::ENABLE_MEDRXIV;
// use config::ENABLE_REDDIT;
pub async fn threads_parts() {
    let mut threads_vec = vec![];
    // if ENABLE_REDDIT {
    //     threads_vec.push(thread::spawn(move || {
    //         fetch::reddit_fetch::reddit_fetch::reddit_part();
    //     }));
    // }
    if ENABLE_ARXIV {
        threads_vec.push(thread::spawn(move || {
            fetch::arxiv_fetch::arxiv_fetch::arxiv_part();
        }));
    }
    // if ENABLE_BIORXIV {
    //     threads_vec.push(thread::spawn(move || {
    //         fetch::biorxiv_fetch::biorxiv_fetch::biorxiv_part();
    //     }));
    // }
    // if ENABLE_MEDRXIV {
    //     threads_vec.push(thread::spawn(move || {
    //         fetch::medrxiv_fetch::medrxiv_fetch::medrxiv_part();
    //     }));
    // }
    for i in threads_vec {
        i.join().unwrap();
    }
}
