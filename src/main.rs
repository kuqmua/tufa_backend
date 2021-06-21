mod fetch {
    pub mod parse_github_html;
    pub mod rss_async_write_fetch_error_logs_into_file;
    pub mod rss_async_write_fetch_error_logs_into_files_wrapper;
    pub mod rss_check_available_providers;
    pub mod rss_check_handled_fetch_status_info;
    pub mod rss_check_provider_status;
    pub mod rss_clean_logs_directory;
    pub mod rss_clean_logs_directory_wrapper;
    pub mod rss_divide_to_equal_for_each_provider;
    pub mod rss_fetch_and_parse_provider_data;
    pub mod rss_fetch_link;
    pub mod rss_filter_fetched_and_parsed_posts;
    pub mod rss_handle_error_status_code;
    pub mod rss_handle_unfiltered_posts;
    pub mod rss_logs_create_dir_if_dont_exists;
    pub mod rss_metainfo_fetch_structures;
    pub mod rss_parse_string_into_struct;
    pub mod rss_part;
    pub mod rss_provider_kind_enum;
    pub mod rss_write_error_logs_into_file_for_provider;
    pub mod rss_write_error_logs_into_file_for_provider_wrapper_checker;
    pub mod info_structures {
        pub mod structs_for_parsing {
            pub mod arxiv_struct_for_parsing;
            pub mod biorxiv_struct_for_parsing;
            pub mod github_struct_for_parsing;
            pub mod habr_struct_for_parsing;
            pub mod medrxiv_struct_for_parsing;
            pub mod reddit_struct_for_parsing;
            pub mod twitter_struct_for_parsing;
        }
        pub mod common_rss_structures;
    }
}
mod check_net {
    pub mod check_link;
    pub mod check_link_metainfo_structures;
    pub mod fetch_link;
}
mod overriding {
    pub mod prints;
}
mod authorization {
    pub mod reddit {
        pub mod reddit_authorization;
    }
}

mod async_tokio_wrapper;
mod check_new_posts_threads_parts;
mod entry;

// use log::LevelFilter;
// use simplelog::{Config, TermLogger, TerminalMode};

fn main() {
    //with logs there is so much spam...
    // TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout).unwrap();
    entry::entry();
}
//mongodb older version working example start
// use futures::executor::block_on;
// use mongodb::{error::Error, options::ClientOptions, Client};

// pub async fn something() -> Result<(), Error> {
//     let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
//     println!("0");
//     let client = Client::with_options(client_options).unwrap();
//     println!("1");
//     let db = client.database("mydb");
//     println!("2");
//     let user_collection = db.collection("users");
//     println!("3 {:#?}", user_collection);
//     Ok(())
// }
// fn main() {
//     block_on(something());
// }
//mongodb older version working example end

// use std::sync::mpsc::channel;
// use std::time::Instant;
// use threadpool::ThreadPool;

// fn main() {
//     let since_fetch = Instant::now();
//     let n_workers = 4;
//     let n_jobs = 8000;
//     let pool = ThreadPool::new(n_workers);

//     let (tx, rx) = channel();
//     for _ in 0..n_jobs {
//         let tx = tx.clone();
//         pool.execute(move || {
//             println!("aaa");
//             tx.send(1)
//                 .expect("channel will be there waiting for the pool");
//         });
//     }

//     assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8000);
//     println!(
//         "in {} sec {} mill",
//         since_fetch.elapsed().as_secs(),
//         since_fetch.elapsed().as_millis()
//     );
// }
