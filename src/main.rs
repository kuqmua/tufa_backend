use std::time::Instant;
mod fetch {
    pub mod handle_error_status_code;
    pub mod metainfo_fetch_structures;
    pub mod rxiv_check_handled_fetch_status_info;
    pub mod rxiv_fetch_and_parse_xml;
    pub mod rxiv_fetch_link;
    pub mod rxiv_filter_fetched_and_parsed_posts;
    pub mod rxiv_kind_enum;
    pub mod rxiv_parse_string_into_struct;
    pub mod rxiv_part;
    pub mod rxiv_structures;
    // pub mod reddit_fetch {
    //     pub mod get_reddit_posts;
    //     pub mod parse_every_children;
    //     pub mod push_names_into_two_layer_result_vec;
    //     pub mod reddit_fetch;
    //     pub mod subreddits_into_urls;
    //     pub mod reddit_json_structs {
    //         pub mod casted;
    //         pub mod used;
    //     }
    // }
}
mod get_group_names {
    pub mod get_arxiv_links;
    pub mod get_biorxiv_links;
    pub mod get_medrxiv_links;
    // pub mod get_subreddits;
}
mod check_net {
    pub mod check_link;
    pub mod check_link_metainfo_structures;
    pub mod fetch_link;
}
mod overriding {
    pub mod prints;
}

// mod authorization {
//     pub mod reddit {
//         pub mod authorization_info;
//         pub mod reddit_authorization;
//     }
// }

mod async_tokio_wrapper;
mod config;
mod entry;
mod threads_parts;

use entry::entry;
// use log::LevelFilter;
// use simplelog::{Config, TermLogger, TerminalMode};
fn main() {
    //c логами реально дохерище спама
    // TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout).unwrap();
    let time = Instant::now();
    entry();
    println!("main done in {} seconds", time.elapsed().as_secs());
}
