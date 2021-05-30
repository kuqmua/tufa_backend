mod fetch {
    // pub mod parse_github_html;
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
mod get_project_information {
    pub mod get_names {
        pub mod get_arxiv_names;
        pub mod get_biorxiv_names;
        pub mod get_github_names;
        pub mod get_habr_names;
        pub mod get_medrxiv_names;
        pub mod get_reddit_names;
        pub mod get_twitter_names;
    }
    pub mod generate_hashmap_links {
        pub mod generate_arxiv_hashmap_links;
        pub mod generate_biorxiv_hashmap_links;
        pub mod generate_github_hashmap_links;
        pub mod generate_habr_hashmap_links;
        pub mod generate_medrxiv_hashmap_links;
        pub mod generate_reddit_hashmap_links;
        pub mod generate_twitter_hashmap_links;
    }
    pub mod get_config {
        pub mod config_structures;
        pub mod get_config_information;
    }
    pub mod get_user_credentials {
        pub mod get_user_credentials_information;
        pub mod user_credentials_structures;
    }
    pub mod get_twitter_providers_names;
    pub mod project_constants;
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

#[cfg(test)]
mod tests {
    pub mod continuous_integration {
        pub mod ci_check_compromised_reddit_auth_info;
        pub mod ci_check_config_files_exists;
    }
    mod tests_constants;
}

#[macro_use]
extern crate lazy_static;

// use log::LevelFilter;
// use simplelog::{Config, TermLogger, TerminalMode};

fn main() {
    //with logs there is so much spam...
    // TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout).unwrap();
    entry::entry();
}

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

// use futures::executor::block_on;
// use std::sync::{Arc, Mutex};

// struct State {
//     count: u64,
// }

// async fn task1(state: &Arc<Mutex<State>>) -> u64 {
//     if let Ok(mut state) = state.lock() {
//         state.count += 1;
//     }
//     println!("task1");
//     1
// }

// async fn task2(state: &Arc<Mutex<State>>) -> u64 {
//     if let Ok(mut state) = state.lock() {
//         state.count += 2;
//     }
//     println!("task2");
//     2
// }

// async fn async_main() {
//     let state = State { count: 0 };
//     let data = Arc::new(Mutex::new(state));
//     let (result1, result2) = futures::join!(task1(&data), task2(&data));
//     println!("result1 {} result2 {}", result1, result2);
//     if let Ok(s) = data.lock() {
//         println!("state {}", s.count);
//     };
// }

// fn main() {
//     block_on(async_main());
// }

// use async_std::task;
// use futures::executor::block_on;
// use futures::future::join_all;
// use std::time::Duration;

// async fn foo(i: u32) -> u32 {
//     task::sleep(Duration::from_secs(5)).await;
//     println!("foo");
//     i
// }
// async fn async_main() {
//     let integers = vec![3, 4, 5];
//     let mut futures = Vec::with_capacity(integers.len());
//     for i in integers {
//         futures.push(foo(i));
//     }
//     let end = join_all(futures).await;
//     println!("{:#?}", end);
// }

// fn main() {
//     block_on(async_main());
// }
