use std::thread;
use std::time::Instant;

mod fetch {
    pub mod arxiv_fetch;
    pub mod biorxiv_fetch;
    pub mod medrxiv_fetch;
    pub mod reddit_fetch;
}
mod get_group_names {
    pub mod get_arxiv_links;
    pub mod get_biorxiv_links;
    pub mod get_medrxiv_links;
    pub mod get_subreddits;
}
mod check_provider {
    pub mod can_i_reach_provider;
}
mod override_prints {
    pub mod override_prints;
}

mod authorization {
    pub mod reddit {
        pub mod authorization_info;
        pub mod reddit_authorization;
    }
}

mod parsing {
    pub mod reddit {
        pub mod parse_reddit {
            pub mod get_reddit_posts;
        }
        pub mod reddit_json_structs {
            pub mod casted;
            pub mod used;
        }
    }
}

mod config;
use config::ARXIV_URL;
use config::BIORXIV_URL;
use config::MEDRXIV_URL;
use config::REDDIT_URL;

fn main() {
    let time = Instant::now();
    let mut threads_vec = vec![];
    threads_vec.push(thread::spawn(move || {
        fetch::reddit_fetch::reddit_part();
    }));
    threads_vec.push(thread::spawn(move || {
        fetch::arxiv_fetch::arxiv_part();
    }));
    threads_vec.push(thread::spawn(move || {
        fetch::biorxiv_fetch::biorxiv_part();
    }));
    threads_vec.push(thread::spawn(move || {
        fetch::medrxiv_fetch::medrxiv_part(); //TODO паника тут!!!
    }));
    for i in threads_vec {
        i.join().unwrap();
    }
    println!("main done in {} seconds", time.elapsed().as_secs());
}
// let biorxiv_vec = biorxiv_part();
// for (key, value) in biorxiv_vec {
//     print!("{:#?}\n", key);
// }
