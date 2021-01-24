use std::thread;
use std::time::Instant;

mod fetch {
    pub mod arxiv_fetch {
        pub mod arxiv_fetch;
        pub mod arxiv_fetch_and_parse_xml;
        pub mod arxiv_structures;
    }
    pub mod biorxiv_fetch {
        pub mod biorxiv_fetch;
        pub mod biorxiv_fetch_and_parse_xml;
        pub mod biorxiv_structures;
    }
    pub mod medrxiv_fetch {
        pub mod medrxiv_fetch;
        pub mod medrxiv_fetch_and_parse_xml;
        pub mod medrxiv_structures;
    }
    pub mod reddit_fetch {
        pub mod get_reddit_posts;
        pub mod reddit_fetch;
        pub mod reddit_json_structs {
            pub mod casted;
            pub mod used;
        }
    }
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

mod config;
use config::ENABLE_ARXIV;
use config::ENABLE_BIORXIV;
use config::ENABLE_MEDRXIV;
use config::ENABLE_REDDIT;

fn main() {
    let time = Instant::now();
    let mut threads_vec = vec![];
    if ENABLE_REDDIT {
        threads_vec.push(thread::spawn(move || {
            fetch::reddit_fetch::reddit_fetch::reddit_part();
        }));
    }
    if ENABLE_ARXIV {
        threads_vec.push(thread::spawn(move || {
            fetch::arxiv_fetch::arxiv_fetch::arxiv_part();
        }));
    }
    if ENABLE_BIORXIV {
        threads_vec.push(thread::spawn(move || {
            fetch::biorxiv_fetch::biorxiv_fetch::biorxiv_part();
        }));
    }
    if ENABLE_MEDRXIV {
        threads_vec.push(thread::spawn(move || {
            fetch::medrxiv_fetch::medrxiv_fetch::medrxiv_part();
        }));
    }
    for i in threads_vec {
        i.join().unwrap();
    }
    println!("main done in {} seconds", time.elapsed().as_secs());
}
