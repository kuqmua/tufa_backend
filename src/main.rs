use std::time::Instant;
mod fetch {
    pub mod arxiv_part {
        pub mod arxiv_fetch;
        pub mod arxiv_fetch_and_parse_xml;
        pub mod arxiv_metainfo_structures;
        pub mod arxiv_structures;
    }
    // pub mod biorxiv_fetch {
    //     pub mod biorxiv_fetch;
    //     pub mod biorxiv_fetch_and_parse_xml;
    //     pub mod biorxiv_structures;
    // }
    // pub mod medrxiv_fetch {
    //     pub mod medrxiv_fetch;
    //     pub mod medrxiv_fetch_and_parse_xml;
    //     pub mod medrxiv_structures;
    // }
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
    // pub mod get_biorxiv_links;
    // pub mod get_medrxiv_links;
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

fn main() {
    let time = Instant::now();
    entry();
    println!("main done in {} seconds", time.elapsed().as_secs());
}
